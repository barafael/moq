//! Endpoint keys on graviola, with RustCrypto's HKDF, which graviola does not expose.

use graviola::{
    aead::AesGcm,
    hashing::{Sha256, hmac::Hmac},
};

use crate::crypto::{self, CryptoError};

/// HMAC-SHA256 with its key schedule computed once.
pub(crate) struct HmacKey(Hmac<Sha256>);

impl HmacKey {
    pub(crate) fn new(key: &[u8]) -> Self {
        Self(Hmac::new(key))
    }
}

impl crypto::HmacKey for HmacKey {
    fn sign(&self, data: &[u8], out: &mut [u8]) {
        let mut hmac = self.0.clone();
        hmac.update(data);
        out.copy_from_slice(hmac.finish().as_ref());
    }

    fn signature_len(&self) -> usize {
        32
    }

    fn verify(&self, data: &[u8], signature: &[u8]) -> Result<(), CryptoError> {
        let mut hmac = self.0.clone();
        hmac.update(data);
        hmac.verify(signature).map_err(|_| CryptoError)
    }
}

/// An HKDF-SHA256 pseudorandom key.
pub(crate) type Prk = hkdf::Hkdf<sha2::Sha256>;

/// Extract a pseudorandom key from `secret` with an empty salt.
pub(crate) fn extract(secret: &[u8]) -> Prk {
    Prk::new(None, secret)
}

impl crypto::HandshakeTokenKey for Prk {
    fn aead_from_hkdf(&self, random_bytes: &[u8]) -> Box<dyn crypto::AeadKey> {
        let mut key = [0u8; 32];
        self.expand(random_bytes, &mut key)
            .expect("32 bytes fit HKDF-SHA256");
        Box::new(ZeroNonceAead(AesGcm::new(&key)))
    }
}

/// AES-256-GCM under a fixed zero nonce: each key seals exactly one token.
struct ZeroNonceAead(AesGcm);

const TAG_LEN: usize = 16;

impl crypto::AeadKey for ZeroNonceAead {
    fn seal(&self, data: &mut Vec<u8>, additional_data: &[u8]) -> Result<(), CryptoError> {
        let mut tag = [0u8; TAG_LEN];
        self.0.encrypt(&[0; 12], additional_data, data, &mut tag);
        data.extend_from_slice(&tag);
        Ok(())
    }

    fn open<'a>(
        &self,
        data: &'a mut [u8],
        additional_data: &[u8],
    ) -> Result<&'a mut [u8], CryptoError> {
        let plain_len = data.len().checked_sub(TAG_LEN).ok_or(CryptoError)?;
        let (plain, tag) = data.split_at_mut(plain_len);
        self.0
            .decrypt(&[0; 12], additional_data, plain, tag)
            .map_err(|_| CryptoError)?;
        Ok(plain)
    }
}
