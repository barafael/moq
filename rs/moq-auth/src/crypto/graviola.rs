use graviola::hashing::{Hash, Sha256, Sha384, Sha512, hmac::Hmac};
use graviola::signing::{ecdsa, eddsa, rsa};
use jsonwebtoken::DecodingKeyKind;
use jsonwebtoken::crypto::{CryptoProvider, JwtSigner, JwtVerifier, KeyUtils};
use jsonwebtoken::errors::{ErrorKind, Result};
use jsonwebtoken::signature::{self, Signer, Verifier};
use jsonwebtoken::{Algorithm, DecodingKey, EncodingKey};
use pkcs1::der::{Decode, Encode, asn1::UintRef};

pub(crate) fn init() -> crate::Result<()> {
	check_cpu()?;
	// The first provider wins, including one installed by the embedding application.
	let _ = PROVIDER.install_default();
	Ok(())
}

fn check_cpu() -> crate::Result<()> {
	// Graviola checks these with assertions on entry, including for hashing.
	// Refuse during key setup instead of panicking while authenticating a peer.
	#[cfg(target_arch = "x86_64")]
	let supported = std::is_x86_feature_detected!("aes")
		&& std::is_x86_feature_detected!("pclmulqdq")
		&& std::is_x86_feature_detected!("bmi1")
		&& std::is_x86_feature_detected!("adx")
		&& std::is_x86_feature_detected!("avx")
		&& std::is_x86_feature_detected!("avx2");
	#[cfg(target_arch = "aarch64")]
	let supported = std::arch::is_aarch64_feature_detected!("neon")
		&& std::arch::is_aarch64_feature_detected!("aes")
		&& std::arch::is_aarch64_feature_detected!("pmull")
		&& std::arch::is_aarch64_feature_detected!("sha2");
	if !supported {
		return Err(crate::Error::Crypto(
			"CPU lacks graviola's required crypto instructions".into(),
		));
	}
	Ok(())
}

pub(crate) fn fill(bytes: &mut [u8]) -> crate::Result<()> {
	Ok(graviola::random::fill(bytes)?)
}

pub(crate) fn generate_rsa() -> crate::Result<Vec<u8>> {
	let key = rsa::SigningKey::generate(rsa::KeySize::Rsa2048)?;
	// A two-prime 2048-bit PKCS#1 key fits within 2048 bytes.
	Ok(key.to_pkcs1_der(&mut [0; 2048])?.to_vec())
}

pub(crate) fn generate_ed25519() -> crate::Result<(Vec<u8>, Vec<u8>)> {
	let key = eddsa::Ed25519SigningKey::generate()?;
	Ok((key.as_seed().to_vec(), key.public_key().as_bytes().to_vec()))
}

pub(crate) fn ed25519_der(seed: &[u8], public: &[u8]) -> crate::Result<Vec<u8>> {
	let key = eddsa::Ed25519SigningKey::from_bytes(seed)?;
	if key.public_key().as_bytes() != public {
		return Err(crate::Error::Crypto(
			"Ed25519 public key does not match its seed".into(),
		));
	}
	Ok(key.to_pkcs8_der(&mut [0; 128])?.to_vec())
}

fn invalid(error: impl std::fmt::Display) -> jsonwebtoken::errors::Error {
	ErrorKind::Provider(error.to_string()).into()
}

struct Signing {
	algorithm: Algorithm,
	key: SigningKey,
}

enum SigningKey {
	Hmac(Vec<u8>),
	P256(Box<ecdsa::SigningKey<ecdsa::P256>>),
	P384(Box<ecdsa::SigningKey<ecdsa::P384>>),
	Ed25519(Box<eddsa::Ed25519SigningKey>),
	Rsa(Box<rsa::SigningKey>),
}

struct Verifying {
	algorithm: Algorithm,
	key: VerifyingKey,
}

enum VerifyingKey {
	Hmac(Vec<u8>),
	P256(Box<ecdsa::VerifyingKey<ecdsa::P256>>),
	P384(Box<ecdsa::VerifyingKey<ecdsa::P384>>),
	Ed25519(Box<eddsa::Ed25519VerifyingKey>),
	Rsa {
		key: Box<rsa::VerifyingKey>,
		signature_len: usize,
	},
}

fn new_signer(algorithm: &Algorithm, key: &EncodingKey) -> Result<Box<dyn JwtSigner>> {
	check_cpu().map_err(invalid)?;
	if !key.family().algorithms().contains(algorithm) {
		return Err(ErrorKind::InvalidKeyFormat.into());
	}
	let bytes = key.as_bytes();
	let key = match algorithm {
		Algorithm::HS256 | Algorithm::HS384 | Algorithm::HS512 => SigningKey::Hmac(bytes.to_vec()),
		Algorithm::ES256 => SigningKey::P256(Box::new(ecdsa::SigningKey::from_pkcs8_der(bytes).map_err(invalid)?)),
		Algorithm::ES384 => SigningKey::P384(Box::new(ecdsa::SigningKey::from_pkcs8_der(bytes).map_err(invalid)?)),
		Algorithm::EdDSA => SigningKey::Ed25519(Box::new(
			eddsa::Ed25519SigningKey::from_pkcs8_der(bytes).map_err(invalid)?,
		)),
		_ => SigningKey::Rsa(Box::new(rsa::SigningKey::from_pkcs1_der(bytes).map_err(invalid)?)),
	};
	Ok(Box::new(Signing {
		algorithm: *algorithm,
		key,
	}))
}

fn new_verifier(algorithm: &Algorithm, key: &DecodingKey) -> Result<Box<dyn JwtVerifier>> {
	check_cpu().map_err(invalid)?;
	if !key.family().algorithms().contains(algorithm) {
		return Err(ErrorKind::InvalidKeyFormat.into());
	}
	let der;
	let bytes = match key.kind() {
		DecodingKeyKind::SecretOrDer(bytes) => bytes,
		DecodingKeyKind::RsaModulusExponent { n, e } => {
			der = pkcs1::RsaPublicKey {
				modulus: UintRef::new(n).map_err(invalid)?,
				public_exponent: UintRef::new(e).map_err(invalid)?,
			}
			.to_der()
			.map_err(invalid)?;
			&der
		}
	};
	let key = match algorithm {
		Algorithm::HS256 | Algorithm::HS384 | Algorithm::HS512 => VerifyingKey::Hmac(bytes.to_vec()),
		Algorithm::ES256 => VerifyingKey::P256(Box::new(
			ecdsa::VerifyingKey::from_x962_uncompressed(bytes).map_err(invalid)?,
		)),
		Algorithm::ES384 => VerifyingKey::P384(Box::new(
			ecdsa::VerifyingKey::from_x962_uncompressed(bytes).map_err(invalid)?,
		)),
		Algorithm::EdDSA => VerifyingKey::Ed25519(Box::new(
			eddsa::Ed25519VerifyingKey::from_bytes(bytes).map_err(invalid)?,
		)),
		_ => VerifyingKey::Rsa {
			key: Box::new(rsa::VerifyingKey::from_pkcs1_der(bytes).map_err(invalid)?),
			signature_len: pkcs1::RsaPublicKey::from_der(bytes)
				.map_err(invalid)?
				.modulus
				.as_bytes()
				.len(),
		},
	};
	Ok(Box::new(Verifying {
		algorithm: *algorithm,
		key,
	}))
}

fn hmac<H: Hash>(key: &[u8], message: &[u8]) -> Hmac<H> {
	let mut hmac = Hmac::new(key);
	hmac.update(message);
	hmac
}

impl Signer<Vec<u8>> for Signing {
	fn try_sign(&self, message: &[u8]) -> std::result::Result<Vec<u8>, signature::Error> {
		let result = match &self.key {
			SigningKey::Hmac(key) => {
				return Ok(match self.algorithm {
					Algorithm::HS256 => hmac::<Sha256>(key, message).finish(),
					Algorithm::HS384 => hmac::<Sha384>(key, message).finish(),
					Algorithm::HS512 => hmac::<Sha512>(key, message).finish(),
					_ => return Err(signature::Error::new()),
				}
				.as_ref()
				.to_vec());
			}
			SigningKey::P256(key) => key.sign::<Sha256>(&[message], &mut [0; 64]).map(<[u8]>::to_vec),
			SigningKey::P384(key) => key.sign::<Sha384>(&[message], &mut [0; 96]).map(<[u8]>::to_vec),
			SigningKey::Ed25519(key) => return Ok(key.sign(message).to_vec()),
			SigningKey::Rsa(key) => {
				let mut output = vec![0; key.modulus_len_bytes()];
				match self.algorithm {
					Algorithm::RS256 => key.sign_pkcs1_sha256(&mut output, message),
					Algorithm::RS384 => key.sign_pkcs1_sha384(&mut output, message),
					Algorithm::RS512 => key.sign_pkcs1_sha512(&mut output, message),
					Algorithm::PS256 => key.sign_pss_sha256(&mut output, message),
					Algorithm::PS384 => key.sign_pss_sha384(&mut output, message),
					Algorithm::PS512 => key.sign_pss_sha512(&mut output, message),
					_ => return Err(signature::Error::new()),
				}
				.map(<[u8]>::to_vec)
			}
		};
		result.map_err(signature::Error::from_source)
	}
}

impl Verifier<Vec<u8>> for Verifying {
	fn verify(&self, message: &[u8], signature: &Vec<u8>) -> std::result::Result<(), signature::Error> {
		let result = match &self.key {
			VerifyingKey::Hmac(key) => match self.algorithm {
				Algorithm::HS256 => hmac::<Sha256>(key, message).verify(signature),
				Algorithm::HS384 => hmac::<Sha384>(key, message).verify(signature),
				Algorithm::HS512 => hmac::<Sha512>(key, message).verify(signature),
				_ => return Err(signature::Error::new()),
			},
			VerifyingKey::P256(key) => key.verify::<Sha256>(&[message], signature),
			VerifyingKey::P384(key) => key.verify::<Sha384>(&[message], signature),
			VerifyingKey::Ed25519(key) => key.verify(signature, message),
			VerifyingKey::Rsa { key, signature_len } => {
				if signature.len() != *signature_len {
					return Err(signature::Error::new());
				}
				match self.algorithm {
					Algorithm::RS256 => key.verify_pkcs1_sha256(signature, message),
					Algorithm::RS384 => key.verify_pkcs1_sha384(signature, message),
					Algorithm::RS512 => key.verify_pkcs1_sha512(signature, message),
					Algorithm::PS256 => key.verify_pss_sha256(signature, message),
					Algorithm::PS384 => key.verify_pss_sha384(signature, message),
					Algorithm::PS512 => key.verify_pss_sha512(signature, message),
					_ => return Err(signature::Error::new()),
				}
			}
		};
		result.map_err(signature::Error::from_source)
	}
}

impl JwtSigner for Signing {
	fn algorithm(&self) -> Algorithm {
		self.algorithm
	}
}

impl JwtVerifier for Verifying {
	fn algorithm(&self) -> Algorithm {
		self.algorithm
	}
}

static PROVIDER: CryptoProvider = CryptoProvider {
	signer_factory: new_signer,
	verifier_factory: new_verifier,
	key_utils: KeyUtils {
		rsa_pub_components_from_private_key: |bytes| {
			let key = pkcs1::RsaPrivateKey::from_der(bytes).map_err(invalid)?;
			Ok((key.modulus.as_bytes().to_vec(), key.public_exponent.as_bytes().to_vec()))
		},
		rsa_pub_components_from_public_key: |bytes| {
			let key = pkcs1::RsaPublicKey::from_der(bytes).map_err(invalid)?;
			Ok((key.modulus.as_bytes().to_vec(), key.public_exponent.as_bytes().to_vec()))
		},
		ec_pub_components_from_private_key: |bytes, algorithm| {
			use jsonwebtoken::jwk::EllipticCurve;
			use p256::elliptic_curve::sec1::ToSec1Point;
			use p256::pkcs8::DecodePrivateKey;
			let (curve, point, width) = match algorithm {
				Algorithm::ES256 => (
					EllipticCurve::P256,
					p256::SecretKey::from_pkcs8_der(bytes)
						.map_err(invalid)?
						.public_key()
						.to_sec1_point(false)
						.as_bytes()
						.to_vec(),
					32,
				),
				Algorithm::ES384 => (
					EllipticCurve::P384,
					p384::SecretKey::from_pkcs8_der(bytes)
						.map_err(invalid)?
						.public_key()
						.to_sec1_point(false)
						.as_bytes()
						.to_vec(),
					48,
				),
				_ => return Err(ErrorKind::InvalidAlgorithm.into()),
			};
			Ok((curve, point[1..1 + width].to_vec(), point[1 + width..].to_vec()))
		},
		ed_pub_components_from_private_key: |bytes, curve| {
			check_cpu().map_err(invalid)?;
			if *curve != jsonwebtoken::jwk::EllipticCurve::Ed25519 {
				return Err(ErrorKind::InvalidAlgorithm.into());
			}
			Ok(eddsa::Ed25519SigningKey::from_pkcs8_der(bytes)
				.map_err(invalid)?
				.public_key()
				.as_bytes()
				.to_vec())
		},
		compute_digest: |bytes, hash| {
			check_cpu().map_err(invalid)?;
			use jsonwebtoken::jwk::ThumbprintHash;
			Ok(match hash {
				ThumbprintHash::SHA256 => Sha256::hash(bytes),
				ThumbprintHash::SHA384 => Sha384::hash(bytes),
				ThumbprintHash::SHA512 => Sha512::hash(bytes),
				_ => return Err(ErrorKind::InvalidAlgorithm.into()),
			}
			.as_ref()
			.to_vec())
		},
	},
};
