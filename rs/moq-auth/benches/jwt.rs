use criterion::{Criterion, criterion_group, criterion_main};
use moq_auth::{Algorithm, Claims, Key};
use std::hint::black_box;

fn jwt(c: &mut Criterion) {
	if std::env::var("MOQ_AUTH_BENCH_PROVIDER").as_deref() == Ok("aws-lc") {
		jsonwebtoken::crypto::aws_lc::DEFAULT_PROVIDER
			.install_default()
			.unwrap();
	}
	let claims = Claims::default();
	for algorithm in [
		Algorithm::HS256,
		Algorithm::HS384,
		Algorithm::HS512,
		Algorithm::ES256,
		Algorithm::ES384,
		Algorithm::EdDSA,
		Algorithm::RS256,
		Algorithm::RS384,
		Algorithm::RS512,
		Algorithm::PS256,
		Algorithm::PS384,
		Algorithm::PS512,
	] {
		let key = Key::generate(algorithm, None).unwrap();
		let verifier = if matches!(algorithm, Algorithm::HS256 | Algorithm::HS384 | Algorithm::HS512) {
			key.clone()
		} else {
			key.to_public().unwrap()
		};
		let token = key.sign(&claims).unwrap();
		c.bench_function(&format!("jwt/{algorithm:?}/sign"), |b| {
			b.iter(|| key.sign(black_box(&claims)).unwrap())
		});
		c.bench_function(&format!("jwt/{algorithm:?}/verify"), |b| {
			b.iter(|| verifier.verify(black_box(&token)).unwrap())
		});
	}
}

criterion_group!(benches, jwt);
criterion_main!(benches);
