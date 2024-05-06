pub use core::{
	iter::successors,
	num::NonZeroU8 as Non0U8,
	ops::{Deref, DerefMut},
};

pub use num_bigint::BigUint as UN;
pub use num_integer::Integer;
pub use num_traits::{One, Zero};

#[derive(Debug, Clone)]
pub struct BitIter {
	i: u64,
	n: UN,
}
impl BitIter {
	#[must_use]
	pub const fn new(n: UN) -> Self {
		Self { i: 0, n }
	}
}
impl Iterator for BitIter {
	type Item = bool;
	fn next(&mut self) -> Option<Self::Item> {
		// bounds-check
		if self.i >= self.n.bits() {
			return None;
		}
		let out = self.n.bit(self.i);
		self.i = self
			.i
			.checked_add(1)
			.unwrap_or_else(|| unreachable!("bounds-check failed"));
		Some(out)
	}
}

#[derive(Debug, Clone)]
/// `Vec`tor of packed `bool`s,
/// built on `BigUint`.
pub struct BitVec(UN);
impl BitVec {
	#[must_use]
	pub const fn new(n: UN) -> Self {
		Self(n)
	}
	#[must_use]
	pub fn clone_inc(&self) -> Self {
		Self(self.0.clone() + 1u8)
	}
}
impl Deref for BitVec {
	type Target = UN;
	fn deref(&self) -> &Self::Target {
		&self.0
	}
}
impl DerefMut for BitVec {
	fn deref_mut(&mut self) -> &mut Self::Target {
		&mut self.0
	}
}
impl IntoIterator for BitVec {
	type Item = bool;
	type IntoIter = BitIter;
	fn into_iter(self) -> Self::IntoIter {
		BitIter::new(self.0)
	}
}
