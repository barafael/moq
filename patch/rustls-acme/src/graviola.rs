//! The slice of *ring*'s API this crate uses: SHA-256 on graviola, and ECDSA P-256 on
//! RustCrypto, since graviola cannot generate an ECDSA key.

pub mod error {
    use std::fmt;

    /// A crypto operation failed.
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct Unspecified;

    impl fmt::Display for Unspecified {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.write_str("Unspecified")
        }
    }

    impl std::error::Error for Unspecified {}

    /// A key could not be parsed.
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct KeyRejected;

    impl fmt::Display for KeyRejected {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.write_str("KeyRejected")
        }
    }

    impl std::error::Error for KeyRejected {}
}

pub mod digest {
    use graviola::hashing::{Hash, HashContext, HashOutput, Sha256};

    /// The one digest algorithm this crate uses.
    pub struct Algorithm(());

    /// SHA-256.
    pub static SHA256: Algorithm = Algorithm(());

    /// A finished digest.
    pub struct Digest(HashOutput);

    impl AsRef<[u8]> for Digest {
        fn as_ref(&self) -> &[u8] {
            self.0.as_ref()
        }
    }

    /// Hash `data` in one call.
    pub fn digest(_: &Algorithm, data: &[u8]) -> Digest {
        Digest(Sha256::hash(data))
    }

    /// An incremental digest.
    pub struct Context(<Sha256 as Hash>::Context);

    impl Context {
        pub fn new(_: &Algorithm) -> Self {
            Self(Sha256::new())
        }

        pub fn update(&mut self, data: &[u8]) {
            self.0.update(data);
        }

        pub fn finish(self) -> Digest {
            Digest(self.0.finish())
        }
    }
}

pub mod rand {
    /// The system RNG; RustCrypto and graviola reach it themselves.
    pub struct SystemRandom(());

    impl SystemRandom {
        pub fn new() -> Self {
            Self(())
        }
    }
}

pub mod signature {
    use super::error::{KeyRejected, Unspecified};
    use super::rand::SystemRandom;
    use p256::ecdsa::signature::Signer;
    use p256::elliptic_curve::Generate;
    use p256::pkcs8::{DecodePrivateKey, EncodePrivateKey};

    /// The one signing algorithm this crate uses.
    pub struct EcdsaSigningAlgorithm(());

    /// ECDSA P-256 with SHA-256 and a fixed-length `r || s` signature, as JWS ES256 wants.
    pub static ECDSA_P256_SHA256_FIXED_SIGNING: EcdsaSigningAlgorithm = EcdsaSigningAlgorithm(());

    /// A key pair's public half.
    pub trait KeyPair {
        type PublicKey: AsRef<[u8]>;

        fn public_key(&self) -> &Self::PublicKey;
    }

    /// Bytes owned by a key or signature.
    pub struct Bytes(Box<[u8]>);

    impl AsRef<[u8]> for Bytes {
        fn as_ref(&self) -> &[u8] {
            &self.0
        }
    }

    /// A P-256 key pair for ES256.
    pub struct EcdsaKeyPair {
        key: p256::ecdsa::SigningKey,
        public: Bytes,
    }

    impl std::fmt::Debug for EcdsaKeyPair {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.debug_struct("EcdsaKeyPair").finish_non_exhaustive()
        }
    }

    impl EcdsaKeyPair {
        /// A fresh key, as PKCS#8.
        pub fn generate_pkcs8(_: &EcdsaSigningAlgorithm, _: &SystemRandom) -> Result<Bytes, Unspecified> {
            let key = p256::ecdsa::SigningKey::generate();
            let der = key.to_pkcs8_der().map_err(|_| Unspecified)?;
            Ok(Bytes(der.as_bytes().into()))
        }

        pub fn from_pkcs8(_: &EcdsaSigningAlgorithm, pkcs8: &[u8]) -> Result<Self, KeyRejected> {
            let key = p256::ecdsa::SigningKey::from_pkcs8_der(pkcs8).map_err(|_| KeyRejected)?;
            let public = Bytes(key.verifying_key().to_sec1_point(false).as_bytes().into());
            Ok(Self { key, public })
        }

        pub fn sign(&self, _: &SystemRandom, message: &[u8]) -> Result<Bytes, Unspecified> {
            let signature: p256::ecdsa::Signature = self.key.try_sign(message).map_err(|_| Unspecified)?;
            Ok(Bytes(signature.to_bytes().as_slice().into()))
        }
    }

    impl KeyPair for EcdsaKeyPair {
        type PublicKey = Bytes;

        fn public_key(&self) -> &Bytes {
            &self.public
        }
    }

    /// A fresh P-256 certificate key for rcgen to sign with.
    pub struct CertKey(EcdsaKeyPair);

    impl CertKey {
        pub fn generate() -> Self {
            let pkcs8 = EcdsaKeyPair::generate_pkcs8(&ECDSA_P256_SHA256_FIXED_SIGNING, &SystemRandom::new())
                .expect("a P-256 key encodes as PKCS#8");
            Self(EcdsaKeyPair::from_pkcs8(&ECDSA_P256_SHA256_FIXED_SIGNING, pkcs8.as_ref()).expect("a fresh key parses"))
        }

        pub fn serialize_der(&self) -> Vec<u8> {
            self.0.key.to_pkcs8_der().expect("a P-256 key encodes as PKCS#8").as_bytes().to_vec()
        }

        pub fn serialize_pem(&self) -> String {
            pem::encode(&pem::Pem::new("PRIVATE KEY", self.serialize_der()))
        }
    }

    impl rcgen::PublicKeyData for CertKey {
        fn der_bytes(&self) -> &[u8] {
            self.0.public.as_ref()
        }

        fn algorithm(&self) -> &'static rcgen::SignatureAlgorithm {
            &rcgen::PKCS_ECDSA_P256_SHA256
        }
    }

    impl rcgen::SigningKey for CertKey {
        fn sign(&self, message: &[u8]) -> Result<Vec<u8>, rcgen::Error> {
            let signature: p256::ecdsa::Signature = self.0.key.try_sign(message).map_err(|_| rcgen::Error::RemoteKeyError)?;
            Ok(signature.to_der().as_bytes().to_vec())
        }
    }
}
