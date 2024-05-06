pub use core::{
	iter::successors,
	num::NonZeroU8 as Non0U8,
	ops::{Deref, DerefMut},
};

pub use num_bigint::BigUint as UN;
pub use num_integer::Integer;
pub use num_traits::{One, Zero};

#[derive(Debug, Clone)]
pub struct BoolIter {
	i: u64,
	n: UN,
}
impl BoolIter {
	pub const fn new(n: UN) -> Self {
		Self { i: 0, n }
	}
}
impl Iterator for BoolIter {
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
pub struct BitVec(UN);
impl BitVec {
	pub const fn new(n: UN) -> Self {
		Self(n)
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
	type IntoIter = BoolIter;
	fn into_iter(self) -> Self::IntoIter {
		BoolIter::new(self.0)
	}
}

const MIN_NON_TRIVIAL_BASE: Non0U8 = match Non0U8::new(3) {
	Some(n) => n,
	_ => unreachable!(),
};

/// Interpret `digits` in base `radix`,
/// and return the numeric value represented by that numeral.
///
/// It assumes little-endian order, so the 1st item must be LSB.
pub fn unpack_as_radix<T: IntoIterator<Item = bool>>(digits: T, radix: Non0U8) -> UN {
	let powers = successors(Some(UN::one()), |b| Some(b * radix.get()));
	digits
		.into_iter()
		.zip(powers)
		.filter_map(|(d, pow)| if d { Some(pow) } else { None })
		.sum()
}

/// Checks if `n` can be written in base `radix`,
/// using only zeros and ones.
pub fn is_0_1(n: UN, radix: Non0U8) -> bool {
	let radix = UN::from(radix.get());
	let n1 = UN::one();
	if radix < UN::from(3u8) {
		return true;
	}
	// IDK how to get rid of `zero` boilerplate
	successors(Some((n, UN::zero())), |(n, _)|
		// 0 & 1 are the same in all (valid) radices
		if n > &n1 {
			Some(n.div_rem(&radix))
		} else {
			None
		}
	)
	.all(|(_, digit)| digit <= n1)
}

/// Checks if `n` can be written using only zeros and ones,
/// in all bases
/// from the minimun non-trivial base (inclusive)
/// to `max_radix` (exclusive).
pub fn is_0_1_all(n: &UN, max_radix: Non0U8) -> bool {
	// would `rev` be more optimal?
	(MIN_NON_TRIVIAL_BASE.get()..max_radix.get()).all(|radix| {
		is_0_1(
			n.clone(),
			Non0U8::new(radix).unwrap_or_else(|| unreachable!()),
		)
	})
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn unpacker_bin_idempotent() {
		assert!((0..=u16::MAX).all(|n| {
			let n = UN::from(n);
			n == unpack_as_radix(
				BitVec::new(n.clone()),
				Non0U8::new(2).unwrap_or_else(|| unreachable!()),
			)
		}));
	}

	#[test]
	fn unpacker_works_non_bin() {
		assert_eq!(
			unpack_as_radix(
				[false, true, false, true],
				Non0U8::new(3).unwrap_or_else(|| unreachable!())
			),
			UN::from(27 + 3u8)
		);
		assert_eq!(
			unpack_as_radix(
				[false, true, false, true],
				Non0U8::new(4).unwrap_or_else(|| unreachable!())
			),
			UN::from(64 + 4u8)
		);
		assert_eq!(
			unpack_as_radix(
				[false, true, true],
				Non0U8::new(5).unwrap_or_else(|| unreachable!())
			),
			UN::from(25 + 5u8)
		);
	}

	#[test]
	fn checker_happy() {
		assert!([(3u8, 3), (4, 3)].into_iter().all(|(n, radix)| is_0_1(
			UN::from(n),
			Non0U8::new(radix).unwrap_or_else(|| unreachable!())
		)));
		assert!([(4, 5), (82000u32, 6)]
			.into_iter()
			.all(|(n, radix)| is_0_1_all(
				&UN::from(n),
				Non0U8::new(radix).unwrap_or_else(|| unreachable!())
			)));
	}

	#[test]
	fn checker_sad() {
		assert!([(2u8, 3), (3, 4)].into_iter().all(|(n, radix)| !is_0_1(
			UN::from(n),
			Non0U8::new(radix).unwrap_or_else(|| unreachable!())
		)));
		assert!([(3u8, 5), (5, 5), (4, 6)]
			.into_iter()
			.all(|(n, radix)| !is_0_1_all(
				&UN::from(n),
				Non0U8::new(radix).unwrap_or_else(|| unreachable!())
			)));
	}
}
