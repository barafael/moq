use aws_lc_rs::encoding::{AsBigEndian, AsDer, Pkcs8V1Der};
use aws_lc_rs::signature::KeyPair;
use p256::pkcs8::der::Decode;

pub(crate) fn init() -> crate::Result<()> {
	let _ = jsonwebtoken::crypto::aws_lc::DEFAULT_PROVIDER.install_default();
	Ok(())
}

pub(crate) fn fill(bytes: &mut [u8]) -> crate::Result<()> {
	Ok(aws_lc_rs::rand::fill(bytes)?)
}

pub(crate) fn generate_rsa() -> crate::Result<Vec<u8>> {
	let key = aws_lc_rs::rsa::KeyPair::generate(aws_lc_rs::rsa::KeySize::Rsa2048)?;
	let der: Pkcs8V1Der<'_> = key.as_der()?;
	let info = p256::pkcs8::PrivateKeyInfoRef::from_der(der.as_ref())?;
	Ok(info.private_key.as_bytes().to_vec())
}

pub(crate) fn generate_ed25519() -> crate::Result<(Vec<u8>, Vec<u8>)> {
	let key = aws_lc_rs::signature::Ed25519KeyPair::generate()?;
	Ok((
		key.seed()?.as_be_bytes()?.as_ref().to_vec(),
		key.public_key().as_ref().to_vec(),
	))
}

pub(crate) fn ed25519_der(seed: &[u8], public: &[u8]) -> crate::Result<Vec<u8>> {
	let key = aws_lc_rs::signature::Ed25519KeyPair::from_seed_and_public_key(seed, public)?;
	Ok(key.to_pkcs8()?.as_ref().to_vec())
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn portable_key_generation() {
		init().unwrap();
		let mut secret = [0; 32];
		fill(&mut secret).unwrap();
		let der = generate_rsa().unwrap();
		let key = aws_lc_rs::signature::RsaKeyPair::from_der(&der).unwrap();
		assert_eq!(key.public_modulus_len(), 256);
		let (seed, public) = generate_ed25519().unwrap();
		let der = ed25519_der(&seed, &public).unwrap();
		let key = aws_lc_rs::signature::Ed25519KeyPair::from_pkcs8(&der).unwrap();
		assert_eq!(key.public_key().as_ref(), public);
	}
}
