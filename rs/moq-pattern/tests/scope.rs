//! Replays the exact Scope contract shared with JavaScript.
use moq_pattern::{Patterns, Scope};
use serde_json::Value;

#[test]
fn shared_vectors() {
	let vectors: Value = serde_json::from_str(include_str!("scope.json")).unwrap();
	for vector in vectors.as_array().unwrap() {
		let patterns = |key: &str| -> Patterns {
			vector[key]
				.as_array()
				.unwrap()
				.iter()
				.map(|value| value.as_str().unwrap().parse().unwrap())
				.collect()
		};
		let scope = Scope::new(patterns("include"), patterns("exclude"));
		assert_eq!(
			scope.is_empty().unwrap(),
			vector["empty"].as_bool().unwrap(),
			"{vector}"
		);
		for (path, expected) in vector["paths"].as_object().unwrap() {
			assert_eq!(scope.matches(path), expected.as_bool().unwrap(), "{vector}: {path}");
			assert_eq!(
				scope.rooted("tenant").unwrap().matches(&format!("tenant/{path}")),
				expected.as_bool().unwrap()
			);
		}
	}
}

#[test]
fn exact_operations_agree_with_a_complete_small_alphabet() {
	let mut paths = vec![String::new()];
	let mut level = paths.clone();
	for _ in 0..5 {
		level = level
			.iter()
			.flat_map(|path| {
				["a", "b", "other"].map(|part| {
					if path.is_empty() {
						part.into()
					} else {
						format!("{path}/{part}")
					}
				})
			})
			.collect();
		paths.extend(level.clone());
	}
	// These predicates distinguish only a, b and any other segment. Five
	// segments separate every head/tail here, including zero-length globstars.
	let patterns = ["", "a", "*", "**", "a/**", "**/a", "a/*", "*/b", "a/**/b"];
	let scopes: Vec<_> = patterns
		.iter()
		.flat_map(|include| {
			["a", "b/**"].map(|exclude| {
				Scope::new(
					Patterns::from(include.parse::<moq_pattern::Pattern>().unwrap()),
					Patterns::from(exclude.parse::<moq_pattern::Pattern>().unwrap()),
				)
			})
		})
		.collect();
	for a in &scopes {
		assert_eq!(a.is_empty().unwrap(), !paths.iter().any(|path| a.matches(path)));
		for b in &scopes {
			assert_eq!(
				a.overlaps(b).unwrap(),
				paths.iter().any(|path| a.matches(path) && b.matches(path)),
				"{a:?} vs {b:?}"
			);
			assert_eq!(
				a.covers(b).unwrap(),
				paths.iter().all(|path| !b.matches(path) || a.matches(path)),
				"{a:?} covers {b:?}"
			);
			let both = a.intersect(b).unwrap();
			for path in &paths {
				assert_eq!(both.matches(path), a.matches(path) && b.matches(path));
			}
		}
	}
}
