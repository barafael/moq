use std::collections::{BTreeSet, HashSet, VecDeque};

use crate::{IntersectionError, InvalidPattern, Pattern, Patterns, Segment};

/// Paths included by a pattern union, except those matched by its exclusions.
#[derive(Clone, Debug, Default, PartialEq, Eq, Hash)]
pub struct Scope {
	include: Patterns,
	exclude: Patterns,
}

/// Why a scope's exact set operation could not be completed safely.
#[derive(Clone, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub enum ScopeError {
	/// An include intersection could not be represented.
	Intersection(IntersectionError),
	/// The exact language search exceeded its complexity budget.
	TooComplex,
}

impl std::fmt::Display for ScopeError {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			Self::Intersection(error) => error.fmt(f),
			Self::TooComplex => f.write_str("scope operation exceeds the complexity limit"),
		}
	}
}

impl std::error::Error for ScopeError {}

impl Scope {
	/// Include these paths, with exclusions taking precedence.
	pub fn new(include: Patterns, exclude: Patterns) -> Self {
		Self { include, exclude }
	}

	/// The positive pattern union, before exclusions.
	pub fn include(&self) -> &Patterns {
		&self.include
	}

	/// The paths removed from the positive union.
	pub fn exclude(&self) -> &Patterns {
		&self.exclude
	}

	/// Whether this exact path is included and not excluded.
	pub fn matches(&self, path: &str) -> bool {
		self.include.matches(path) && !self.exclude.matches(path)
	}

	/// The paths in both scopes; either scope's exclusions still apply.
	pub fn intersect(&self, other: &Self) -> Result<Self, ScopeError> {
		let include = self
			.include
			.intersect(&other.include)
			.map_err(ScopeError::Intersection)?;
		let exclude = self.exclude.iter().chain(other.exclude.iter()).cloned().collect();
		Ok(Self::new(include, exclude))
	}

	/// Place both the included and excluded paths beneath this root.
	pub fn rooted(&self, root: &str) -> Result<Self, InvalidPattern> {
		Ok(Self::new(self.include.rooted(root)?, self.exclude.rooted(root)?))
	}

	/// Name the included and excluded paths relative to this root.
	pub fn rebase(&self, root: &str) -> Self {
		Self::new(self.include.rebase(root), self.exclude.rebase(root))
	}

	/// Whether no path is granted, including exclusions that jointly cover it.
	pub fn is_empty(&self) -> Result<bool, ScopeError> {
		if self.exclude.is_empty() {
			return Ok(self.include.is_empty());
		}
		if self.exclude.covers(&self.include) {
			return Ok(true);
		}
		// One pattern's containment is exact; only a union needs language search.
		if self.exclude.len() == 1 || self.include.iter().any(|pattern| !self.exclude.overlaps(pattern)) {
			return Ok(false);
		}
		Ok(!witness(&[self], |accepted| accepted[0])?)
	}

	/// Whether at least one path is granted by both scopes.
	pub fn overlaps(&self, other: &Self) -> Result<bool, ScopeError> {
		if self.exclude.is_empty() && other.exclude.is_empty() {
			return Ok(self.include.iter().any(|pattern| other.include.overlaps(pattern)));
		}
		Ok(!self.intersect(other)?.is_empty()?)
	}

	/// Whether every path granted by the other scope is granted here.
	pub fn covers(&self, other: &Self) -> Result<bool, ScopeError> {
		if self.exclude.is_empty() && self.include.covers(&other.include) {
			return Ok(true);
		}
		Ok(!witness(&[self, other], |accepted| accepted[1] && !accepted[0])?)
	}
}

impl From<Patterns> for Scope {
	fn from(include: Patterns) -> Self {
		Self::new(include, Patterns::new())
	}
}

impl From<&Patterns> for Scope {
	fn from(include: &Patterns) -> Self {
		include.clone().into()
	}
}

impl From<&Scope> for Scope {
	fn from(scope: &Scope) -> Self {
		scope.clone()
	}
}

impl From<Pattern> for Scope {
	fn from(include: Pattern) -> Self {
		Patterns::from(include).into()
	}
}

// A product search preserves the negative language rather than attempting to
// spell a complement using positive glob patterns. The same budget as positive
// intersection bounds both determinization steps; refusal never drops a deny.
const BUDGET: usize = Pattern::MAX_INTERSECTIONS;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
enum Letter {
	Exact(char),
	Star,
}

fn closure(tokens: &[Letter], positions: &mut BTreeSet<usize>) {
	let mut pending: Vec<_> = positions.iter().copied().collect();
	while let Some(at) = pending.pop() {
		if tokens.get(at) == Some(&Letter::Star) && positions.insert(at + 1) {
			pending.push(at + 1);
		}
	}
}

// Every segment predicate is a literal or prefix*suffix. A finite alphabet of
// its literal characters plus one other character distinguishes every possible
// transition, including Unicode, without sampling possible path strings.
fn segment_masks(segments: &[Segment]) -> Result<Vec<Vec<bool>>, ScopeError> {
	let tokens: Vec<Vec<Letter>> = segments
		.iter()
		.map(|segment| match segment {
			Segment::Literal(value) => value.chars().map(Letter::Exact).collect(),
			Segment::Wildcard => vec![Letter::Star],
			Segment::Partial { prefix, suffix } => prefix
				.chars()
				.map(Letter::Exact)
				.chain([Letter::Star])
				.chain(suffix.chars().map(Letter::Exact))
				.collect(),
			Segment::Globstar => unreachable!("globstars consume whole segments"),
		})
		.collect();
	let mut alphabet: BTreeSet<char> = tokens
		.iter()
		.flatten()
		.filter_map(|letter| match letter {
			Letter::Exact(value) => Some(*value),
			Letter::Star => None,
		})
		.collect();
	let other = (0..=char::MAX as u32)
		.filter_map(char::from_u32)
		.find(|value| *value != '/' && !alphabet.contains(value))
		.expect("finite literal alphabet");
	alphabet.insert(other);
	let initial: Vec<BTreeSet<usize>> = tokens
		.iter()
		.map(|tokens| {
			let mut positions = BTreeSet::from([0]);
			closure(tokens, &mut positions);
			positions
		})
		.collect();
	let mut seen = HashSet::from([initial.clone()]);
	let mut pending = VecDeque::from([initial]);
	let mut masks = BTreeSet::new();
	while let Some(state) = pending.pop_front() {
		for letter in &alphabet {
			let next: Vec<BTreeSet<usize>> = tokens
				.iter()
				.zip(&state)
				.map(|(tokens, positions)| {
					let mut next = BTreeSet::new();
					for at in positions {
						match tokens.get(*at) {
							Some(Letter::Exact(value)) if value == letter => {
								next.insert(at + 1);
							}
							Some(Letter::Star) => {
								next.insert(*at);
							}
							_ => {}
						}
					}
					closure(tokens, &mut next);
					next
				})
				.collect();
			// Only nonempty strings are path segments, even when a wildcard's
			// initial state accepts the empty string.
			masks.insert(
				next.iter()
					.zip(&tokens)
					.map(|(state, tokens)| state.contains(&tokens.len()))
					.collect(),
			);
			if seen.insert(next.clone()) {
				if seen.len() > BUDGET {
					return Err(ScopeError::TooComplex);
				}
				pending.push_back(next);
			}
		}
	}
	Ok(masks.into_iter().collect())
}

fn pattern_closure(pattern: &Pattern, mut state: u64) -> u64 {
	for (at, segment) in pattern.segments().iter().enumerate() {
		if *segment == Segment::Globstar && state & (1 << at) != 0 {
			state |= 1 << (at + 1);
		}
	}
	state
}

fn witness(scopes: &[&Scope], accepts: impl Fn(&[bool]) -> bool) -> Result<bool, ScopeError> {
	let patterns: Vec<_> = scopes
		.iter()
		.flat_map(|scope| scope.include.iter().chain(scope.exclude.iter()))
		.collect();
	let mut segments = Vec::new();
	for pattern in &patterns {
		for segment in pattern.segments() {
			if *segment != Segment::Globstar && !segments.contains(segment) {
				segments.push(segment.clone());
			}
		}
	}
	let masks = segment_masks(&segments)?;
	let initial: Vec<_> = patterns.iter().map(|pattern| pattern_closure(pattern, 1)).collect();
	let mut seen = HashSet::from([initial.clone()]);
	let mut pending = VecDeque::from([initial]);
	while let Some(state) = pending.pop_front() {
		let matched: Vec<_> = state
			.iter()
			.zip(&patterns)
			.map(|(state, pattern)| state & (1 << pattern.segments().len()) != 0)
			.collect();
		let mut at = 0;
		let accepted: Vec<_> = scopes
			.iter()
			.map(|scope| {
				let include = matched[at..at + scope.include.len()].iter().any(|value| *value);
				at += scope.include.len();
				let exclude = matched[at..at + scope.exclude.len()].iter().any(|value| *value);
				at += scope.exclude.len();
				include && !exclude
			})
			.collect();
		if accepts(&accepted) {
			return Ok(true);
		}
		for mask in &masks {
			let next: Vec<_> = patterns
				.iter()
				.zip(&state)
				.map(|(pattern, state)| {
					let mut next = 0;
					for (at, segment) in pattern.segments().iter().enumerate() {
						if state & (1 << at) == 0 {
							continue;
						}
						if *segment == Segment::Globstar {
							next |= 1 << at;
						} else if mask[segments.iter().position(|value| value == segment).unwrap()] {
							next |= 1 << (at + 1);
						}
					}
					pattern_closure(pattern, next)
				})
				.collect();
			if seen.insert(next.clone()) {
				if seen.len() > BUDGET {
					return Err(ScopeError::TooComplex);
				}
				pending.push_back(next);
			}
		}
	}
	Ok(false)
}

#[cfg(test)]
mod tests {
	use super::*;

	fn scope(include: &[&str], exclude: &[&str]) -> Scope {
		let patterns = |values: &[&str]| values.iter().map(|value| value.parse().unwrap()).collect();
		Scope::new(patterns(include), patterns(exclude))
	}

	#[test]
	fn exclusions_win_and_rebase_with_includes() {
		let allowed = scope(&["**"], &["internal/**"]);
		assert!(!allowed.matches("internal"));
		assert!(!allowed.matches("internal/a"));
		assert!(allowed.matches("internalized/a"));
		assert!(allowed.matches(".private/a"));
		let rooted = allowed.rooted("tenant").unwrap();
		assert_eq!(rooted.rebase("tenant"), allowed);
		assert!(rooted.rebase("tenant/internal").is_empty().unwrap());
	}

	#[test]
	fn joint_exclusions_leave_no_witness() {
		let empty = scope(&["room/**"], &["room", "room/*", "room/*/*/**"]);
		assert!(empty.is_empty().unwrap());
		assert!(!empty.overlaps(&scope(&["**"], &[])).unwrap());
		assert!(Scope::default().covers(&empty).unwrap());
	}

	#[test]
	fn intersections_keep_both_denials() {
		let a = scope(&["**"], &["a/**"]);
		let b = scope(&["**/chat"], &["b/**"]);
		let both = a.intersect(&b).unwrap();
		for path in ["", "a", "a/chat", "b/chat", "c/chat", "chat", "c/video"] {
			assert_eq!(both.matches(path), a.matches(path) && b.matches(path), "{path}");
		}
		assert!(a.covers(&both).unwrap());
		assert!(!both.covers(&a).unwrap());
	}

	#[test]
	fn partial_segments_and_unicode_are_not_sampled() {
		let a = scope(&["pre*suf"], &["presuf"]);
		assert!(!a.is_empty().unwrap());
		assert!(!a.overlaps(&scope(&["presuf"], &[])).unwrap());
		assert!(a.overlaps(&scope(&["pre🦀suf"], &[])).unwrap());
		assert!(scope(&["🦀*🐟"], &["🦀*🐟"]).is_empty().unwrap());
	}

	#[test]
	fn complex_search_refuses_instead_of_dropping_denials() {
		let long = "a".repeat(Pattern::MAX_INTERSECTIONS + 1);
		let restricted = scope(&["**"], &[&long, "b"]);
		assert_eq!(restricted.is_empty(), Err(ScopeError::TooComplex));
		assert!(!restricted.matches(&long));
		assert!(!restricted.matches("b"));
	}
}
