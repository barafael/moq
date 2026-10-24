//! graviola against aws-lc-rs through rustls: QUIC packet protection per cipher suite, and a
//! full TLS 1.3 handshake. QUIC carries the same handshake in CRYPTO frames, so its cost is
//! the crypto a QUIC connection pays before its first stream byte.

use std::sync::Arc;

use criterion::{BenchmarkId, Criterion, Throughput, criterion_group, criterion_main};
use rustls::{
    CipherSuite, ClientConfig, ClientConnection, NamedGroup, RootCertStore, ServerConfig,
    ServerConnection, Side,
    crypto::CryptoProvider,
    pki_types::{CertificateDer, PrivateKeyDer, PrivatePkcs8KeyDer},
    quic::{Keys, Version},
};

fn providers() -> [(&'static str, CryptoProvider); 2] {
    [
        ("aws-lc-rs", rustls::crypto::aws_lc_rs::default_provider()),
        (
            "graviola",
            moq_quic::crypto::rustls::graviola::default_provider(),
        ),
    ]
}

/// One short-header packet's protection: seal the payload, then mask the header, and back.
fn bench_packet(c: &mut Criterion) {
    let mut group = c.benchmark_group("packet");
    for suite in [
        CipherSuite::TLS13_AES_128_GCM_SHA256,
        CipherSuite::TLS13_AES_256_GCM_SHA384,
        CipherSuite::TLS13_CHACHA20_POLY1305_SHA256,
    ] {
        for size in [64usize, 1200] {
            group.throughput(Throughput::Bytes(size as u64));
            for (name, provider) in providers() {
                let tls13 = provider
                    .cipher_suites
                    .iter()
                    .find_map(|s| s.tls13().filter(|t| t.common.suite == suite))
                    .expect("a TLS 1.3 suite");
                let quic = tls13.quic.expect("a QUIC-capable suite");
                // Initial-style keys, derived at each suite's own key length.
                let keys = Keys::initial(Version::V1, tls13, quic, &[1; 8], Side::Client);
                let (packet, header) = (keys.local.packet, keys.local.header);

                let id = BenchmarkId::new(format!("{suite:?}/{name}"), size);
                let mut buf = vec![0u8; 4 + size + 16];
                let mut pn = 0u64;
                group.bench_function(id, |b| {
                    b.iter(|| {
                        pn += 1;
                        buf[0] = 0x43;
                        let (head, body) = buf.split_at_mut(4);
                        let (payload, tag_out) = body.split_at_mut(size);
                        let tag = packet.encrypt_in_place(pn, head, payload).unwrap();
                        tag_out.copy_from_slice(tag.as_ref());
                        let (first, rest) = buf.split_first_mut().unwrap();
                        let (number, rest) = rest.split_at_mut(3);
                        header
                            .encrypt_in_place(&rest[1..17], first, number)
                            .unwrap();

                        let (first, rest) = buf.split_first_mut().unwrap();
                        let (number, rest) = rest.split_at_mut(3);
                        header
                            .decrypt_in_place(&rest[1..17], first, number)
                            .unwrap();
                        let (head, body) = buf.split_at_mut(4);
                        packet.decrypt_in_place(pn, head, body).unwrap().len()
                    })
                });
            }
        }
    }
    group.finish();
}

/// A full TLS 1.3 handshake in memory: an ECDSA P-256 server certificate the client verifies,
/// under the provider's preferred key exchange and under X25519 alone.
fn bench_handshake(c: &mut Criterion) {
    let certified = rcgen::generate_simple_self_signed(vec!["localhost".into()]).unwrap();
    let cert = CertificateDer::from(certified.cert.der().to_vec());
    let key = PrivatePkcs8KeyDer::from(certified.signing_key.serialize_der());
    let mut roots = RootCertStore::empty();
    roots.add(cert.clone()).unwrap();
    let roots = Arc::new(roots);

    let mut group = c.benchmark_group("handshake");
    for (name, provider) in providers() {
        for kx in [None, Some(NamedGroup::X25519)] {
            let mut provider = provider.clone();
            if let Some(kx) = kx {
                provider.kx_groups.retain(|group| group.name() == kx);
            }
            let kx_name = format!("{:?}", provider.kx_groups[0].name());
            let provider = Arc::new(provider);
            let server = Arc::new(
                ServerConfig::builder_with_provider(provider.clone())
                    .with_protocol_versions(&[&rustls::version::TLS13])
                    .unwrap()
                    .with_no_client_auth()
                    .with_single_cert(vec![cert.clone()], PrivateKeyDer::Pkcs8(key.clone_key()))
                    .unwrap(),
            );
            let client = Arc::new(
                ClientConfig::builder_with_provider(provider)
                    .with_protocol_versions(&[&rustls::version::TLS13])
                    .unwrap()
                    .with_root_certificates(roots.clone())
                    .with_no_client_auth(),
            );

            group.bench_function(BenchmarkId::new(name, kx_name), |b| {
                b.iter(|| handshake(&client, &server))
            });
        }
    }
    group.finish();
}

fn handshake(client: &Arc<ClientConfig>, server: &Arc<ServerConfig>) {
    let mut client =
        ClientConnection::new(client.clone(), "localhost".try_into().unwrap()).unwrap();
    let mut server = ServerConnection::new(server.clone()).unwrap();
    let mut wire = Vec::new();
    while client.is_handshaking() || server.is_handshaking() {
        wire.clear();
        client.write_tls(&mut wire).unwrap();
        let mut rd = wire.as_slice();
        while !rd.is_empty() {
            server.read_tls(&mut rd).unwrap();
            server.process_new_packets().unwrap();
        }
        wire.clear();
        server.write_tls(&mut wire).unwrap();
        let mut rd = wire.as_slice();
        while !rd.is_empty() {
            client.read_tls(&mut rd).unwrap();
            client.process_new_packets().unwrap();
        }
    }
}

criterion_group!(benches, bench_packet, bench_handshake);
criterion_main!(benches);
