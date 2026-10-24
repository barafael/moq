//! A QUIC-capable rustls provider on graviola.
//!
//! rustls-graviola leaves its TLS 1.3 suites without QUIC keys, so this adds them: graviola
//! seals packets, and RustCrypto's AES and ChaCha20 compute header protection masks, which
//! graviola does not expose.

use std::sync::LazyLock;

use aes::cipher::{BlockCipherEncrypt, KeyInit, KeyIvInit, StreamCipher, StreamCipherSeek};
use graviola::aead::{AesGcm, ChaCha20Poly1305};
use rustls::{
    CipherSuite, Error, SupportedCipherSuite, Tls13CipherSuite,
    crypto::{
        CryptoProvider,
        cipher::{AeadKey, Iv, Nonce},
    },
    quic,
};

/// rustls-graviola's provider, with QUIC keys on its TLS 1.3 suites.
pub fn default_provider() -> CryptoProvider {
    let mut provider = rustls_graviola::default_provider();
    for suite in &mut provider.cipher_suites {
        if let Some(quic) = quic_suite(suite.suite()) {
            *suite = SupportedCipherSuite::Tls13(quic);
        }
    }
    provider
}

fn quic_suite(suite: CipherSuite) -> Option<&'static Tls13CipherSuite> {
    static AES_128_GCM: LazyLock<Tls13CipherSuite> = LazyLock::new(|| {
        with_quic(
            rustls_graviola::suites::TLS13_AES_128_GCM_SHA256,
            &Algorithm::AES_128_GCM,
        )
    });
    static AES_256_GCM: LazyLock<Tls13CipherSuite> = LazyLock::new(|| {
        with_quic(
            rustls_graviola::suites::TLS13_AES_256_GCM_SHA384,
            &Algorithm::AES_256_GCM,
        )
    });
    static CHACHA20_POLY1305: LazyLock<Tls13CipherSuite> = LazyLock::new(|| {
        with_quic(
            rustls_graviola::suites::TLS13_CHACHA20_POLY1305_SHA256,
            &Algorithm::CHACHA20_POLY1305,
        )
    });
    Some(match suite {
        CipherSuite::TLS13_AES_128_GCM_SHA256 => &AES_128_GCM,
        CipherSuite::TLS13_AES_256_GCM_SHA384 => &AES_256_GCM,
        CipherSuite::TLS13_CHACHA20_POLY1305_SHA256 => &CHACHA20_POLY1305,
        _ => return None,
    })
}

fn with_quic(suite: SupportedCipherSuite, quic: &'static Algorithm) -> Tls13CipherSuite {
    let suite = suite.tls13().expect("a TLS 1.3 suite");
    Tls13CipherSuite {
        common: rustls::crypto::CipherSuiteCommon {
            suite: suite.common.suite,
            hash_provider: suite.common.hash_provider,
            confidentiality_limit: suite.common.confidentiality_limit,
        },
        hkdf_provider: suite.hkdf_provider,
        aead_alg: suite.aead_alg,
        quic: Some(quic),
    }
}

#[derive(Clone, Copy)]
enum Cipher {
    Aes128Gcm,
    Aes256Gcm,
    ChaCha20Poly1305,
}

struct Algorithm {
    cipher: Cipher,
    // RFC 9001 section 6.6, as rustls sets them for its own providers.
    confidentiality_limit: u64,
    integrity_limit: u64,
}

impl Algorithm {
    const AES_128_GCM: Self = Self {
        cipher: Cipher::Aes128Gcm,
        confidentiality_limit: 1 << 23,
        integrity_limit: 1 << 52,
    };
    const AES_256_GCM: Self = Self {
        cipher: Cipher::Aes256Gcm,
        confidentiality_limit: 1 << 23,
        integrity_limit: 1 << 52,
    };
    const CHACHA20_POLY1305: Self = Self {
        cipher: Cipher::ChaCha20Poly1305,
        confidentiality_limit: u64::MAX,
        integrity_limit: 1 << 36,
    };
}

impl quic::Algorithm for Algorithm {
    fn packet_key(&self, key: AeadKey, iv: Iv) -> Box<dyn quic::PacketKey> {
        let aead = match self.cipher {
            Cipher::Aes128Gcm | Cipher::Aes256Gcm => Aead::AesGcm(AesGcm::new(key.as_ref())),
            Cipher::ChaCha20Poly1305 => Aead::ChaCha20Poly1305(ChaCha20Poly1305::new(
                key.as_ref().try_into().expect("a 32-byte ChaCha20 key"),
            )),
        };
        Box::new(PacketKey {
            aead,
            iv,
            confidentiality_limit: self.confidentiality_limit,
            integrity_limit: self.integrity_limit,
        })
    }

    fn header_protection_key(&self, key: AeadKey) -> Box<dyn quic::HeaderProtectionKey> {
        let key = key.as_ref();
        Box::new(match self.cipher {
            Cipher::Aes128Gcm => {
                HeaderProtectionKey::Aes128(aes::Aes128::new_from_slice(key).expect("AES-128 key"))
            }
            Cipher::Aes256Gcm => {
                HeaderProtectionKey::Aes256(aes::Aes256::new_from_slice(key).expect("AES-256 key"))
            }
            Cipher::ChaCha20Poly1305 => {
                HeaderProtectionKey::ChaCha20(key.try_into().expect("a 32-byte ChaCha20 key"))
            }
        })
    }

    fn aead_key_len(&self) -> usize {
        match self.cipher {
            Cipher::Aes128Gcm => 16,
            Cipher::Aes256Gcm | Cipher::ChaCha20Poly1305 => 32,
        }
    }
}

enum Aead {
    AesGcm(AesGcm),
    ChaCha20Poly1305(ChaCha20Poly1305),
}

const TAG_LEN: usize = 16;

struct PacketKey {
    aead: Aead,
    iv: Iv,
    confidentiality_limit: u64,
    integrity_limit: u64,
}

impl PacketKey {
    fn seal(&self, nonce: Nonce, header: &[u8], payload: &mut [u8]) -> quic::Tag {
        let mut tag = [0u8; TAG_LEN];
        match &self.aead {
            Aead::AesGcm(aead) => aead.encrypt(&nonce.0, header, payload, &mut tag),
            Aead::ChaCha20Poly1305(aead) => aead.encrypt(&nonce.0, header, payload, &mut tag),
        }
        quic::Tag::from(&tag[..])
    }

    fn open<'a>(
        &self,
        nonce: Nonce,
        header: &[u8],
        payload: &'a mut [u8],
    ) -> Result<&'a [u8], Error> {
        let plain_len = payload
            .len()
            .checked_sub(TAG_LEN)
            .ok_or(Error::DecryptError)?;
        let (plain, tag) = payload.split_at_mut(plain_len);
        match &self.aead {
            Aead::AesGcm(aead) => aead.decrypt(&nonce.0, header, plain, tag),
            Aead::ChaCha20Poly1305(aead) => aead.decrypt(&nonce.0, header, plain, tag),
        }
        .map_err(|_| Error::DecryptError)?;
        Ok(plain)
    }
}

impl quic::PacketKey for PacketKey {
    fn encrypt_in_place(
        &self,
        packet_number: u64,
        header: &[u8],
        payload: &mut [u8],
    ) -> Result<quic::Tag, Error> {
        Ok(self.seal(Nonce::new(&self.iv, packet_number), header, payload))
    }

    fn encrypt_in_place_for_path(
        &self,
        path_id: u32,
        packet_number: u64,
        header: &[u8],
        payload: &mut [u8],
    ) -> Result<quic::Tag, Error> {
        let nonce = Nonce::for_path(path_id, &self.iv, packet_number);
        Ok(self.seal(nonce, header, payload))
    }

    fn decrypt_in_place<'a>(
        &self,
        packet_number: u64,
        header: &[u8],
        payload: &'a mut [u8],
    ) -> Result<&'a [u8], Error> {
        self.open(Nonce::new(&self.iv, packet_number), header, payload)
    }

    fn decrypt_in_place_for_path<'a>(
        &self,
        path_id: u32,
        packet_number: u64,
        header: &[u8],
        payload: &'a mut [u8],
    ) -> Result<&'a [u8], Error> {
        let nonce = Nonce::for_path(path_id, &self.iv, packet_number);
        self.open(nonce, header, payload)
    }

    fn tag_len(&self) -> usize {
        TAG_LEN
    }

    fn confidentiality_limit(&self) -> u64 {
        self.confidentiality_limit
    }

    fn integrity_limit(&self) -> u64 {
        self.integrity_limit
    }
}

enum HeaderProtectionKey {
    Aes128(aes::Aes128),
    Aes256(aes::Aes256),
    ChaCha20([u8; 32]),
}

const SAMPLE_LEN: usize = 16;

impl HeaderProtectionKey {
    /// The five-byte mask of RFC 9001 sections 5.4.3 and 5.4.4.
    fn mask(&self, sample: &[u8]) -> Result<[u8; 5], Error> {
        let sample: &[u8; SAMPLE_LEN] = sample
            .try_into()
            .map_err(|_| Error::General("sample of invalid length".into()))?;
        let mut mask = [0u8; 5];
        match self {
            Self::Aes128(key) => {
                let mut block = (*sample).into();
                key.encrypt_block(&mut block);
                mask.copy_from_slice(&block[..5]);
            }
            Self::Aes256(key) => {
                let mut block = (*sample).into();
                key.encrypt_block(&mut block);
                mask.copy_from_slice(&block[..5]);
            }
            Self::ChaCha20(key) => {
                let (counter, nonce) = sample.split_at(4);
                let counter = u32::from_le_bytes(counter.try_into().unwrap());
                let mut chacha = chacha20::ChaCha20::new(key.into(), nonce.try_into().unwrap());
                chacha.seek(u64::from(counter) * 64);
                chacha.apply_keystream(&mut mask);
            }
        }
        Ok(mask)
    }

    // RFC 9001 section 5.4.1, as rustls applies it for its own providers.
    fn xor_in_place(
        &self,
        sample: &[u8],
        first: &mut u8,
        packet_number: &mut [u8],
        masked: bool,
    ) -> Result<(), Error> {
        let mask = self.mask(sample)?;
        let (first_mask, pn_mask) = mask.split_first().unwrap();
        if packet_number.len() > pn_mask.len() {
            return Err(Error::General("packet number too long".into()));
        }

        const LONG_HEADER_FORM: u8 = 0x80;
        let bits = match *first & LONG_HEADER_FORM == LONG_HEADER_FORM {
            true => 0x0f,
            false => 0x1f,
        };
        let first_plain = match masked {
            true => *first ^ (first_mask & bits),
            false => *first,
        };
        let pn_len = (first_plain & 0x03) as usize + 1;

        *first ^= first_mask & bits;
        for (dst, m) in packet_number.iter_mut().zip(pn_mask).take(pn_len) {
            *dst ^= m;
        }
        Ok(())
    }
}

impl quic::HeaderProtectionKey for HeaderProtectionKey {
    fn encrypt_in_place(
        &self,
        sample: &[u8],
        first: &mut u8,
        packet_number: &mut [u8],
    ) -> Result<(), Error> {
        self.xor_in_place(sample, first, packet_number, false)
    }

    fn decrypt_in_place(
        &self,
        sample: &[u8],
        first: &mut u8,
        packet_number: &mut [u8],
    ) -> Result<(), Error> {
        self.xor_in_place(sample, first, packet_number, true)
    }

    fn sample_len(&self) -> usize {
        SAMPLE_LEN
    }
}

#[cfg(test)]
mod tests {
    use hex_literal::hex;
    use rustls::quic::{Algorithm as _, HeaderProtectionKey as _, PacketKey as _};

    use super::*;

    /// RFC 9001 appendix A.2: the client Initial's AES-128 header protection.
    #[test]
    fn aes_128_mask() {
        let key = HeaderProtectionKey::Aes128(
            aes::Aes128::new_from_slice(&hex!("9f50449e04a0e810283a1e9933adedd2")).unwrap(),
        );
        let mask = key.mask(&hex!("d1b1c98dd7689fb8ec11d242b123dc9b")).unwrap();
        assert_eq!(mask, hex!("437b9aec36"));
    }

    /// RFC 9001 appendix A.5: a ChaCha20-Poly1305 short header packet, end to end.
    #[test]
    fn chacha20_short_header_packet() {
        let alg = Algorithm::CHACHA20_POLY1305;
        let packet_key = alg.packet_key(
            AeadKey::from(hex!(
                "c6d98ff3441c3fe1b2182094f69caa2ed4b716b65488960a7a984979fb23e1c8"
            )),
            Iv::new(hex!("e0459b3474bdd0e44a41c144")),
        );
        let header_key = alg.header_protection_key(AeadKey::from(hex!(
            "25a282b9e82f06f21f488917a4fc8f1b73573685608597d0efcb076b0ab7a7a4"
        )));

        let mut packet = hex!("4200bff4 01 00000000000000000000000000000000").to_vec();
        let (header, payload) = packet.split_at_mut(4);
        let tag = packet_key
            .encrypt_in_place(654360564, header, &mut payload[..1])
            .unwrap();
        payload[1..].copy_from_slice(tag.as_ref());
        let (first, rest) = packet.split_first_mut().unwrap();
        let (pn, payload) = rest.split_at_mut(3);
        header_key
            .encrypt_in_place(&payload[1..17], first, pn)
            .unwrap();
        assert_eq!(packet, hex!("4cfe4189655e5cd55c41f69080575d7999c25a5bfb"));

        let (first, rest) = packet.split_first_mut().unwrap();
        let (pn, payload) = rest.split_at_mut(3);
        header_key
            .decrypt_in_place(&payload[1..17], first, pn)
            .unwrap();
        let (header, payload) = packet.split_at_mut(4);
        assert_eq!(header, hex!("4200bff4"));
        let plain = packet_key
            .decrypt_in_place(654360564, header, payload)
            .unwrap();
        assert_eq!(plain, [1]);
    }
}
