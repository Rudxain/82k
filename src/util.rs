pub use core::num::NonZeroU8 as Non0U8;
pub use num_bigint::BigUint as UN;
pub use num_integer::Integer;

const MIN_NON_TRIVIAL_BASE: Non0U8 = match Non0U8::new(3) {
	Some(n) => n,
	_ => unreachable!(),
};

/// Interpret the bits of `digits` as digits in base `radix`,
/// and return the numeric value represented by that numeral.
pub fn unpack_as_radix(digits: &UN, radix: Non0U8) -> UN {
	let mut out = UN::default();
	let mut pow = UN::from(1u8);

	// LSb
	debug_assert_eq!(digits.bit(0), digits.is_odd());
	for i in 0..digits.bits() {
		if digits.bit(i) {
			out += pow.clone();
		}
		pow *= radix.get();
	}
	out
}

/// Checks if `n` can be written in base `radix`,
/// using only zeros and ones.
pub fn is_0_1(mut n: UN, radix: Non0U8) -> bool {
	let radix = UN::from(radix.get());
	let n1 = UN::from(1u8);

	// 1 is just "1" in any radix
	while n > n1 {
		let digit;
		(n, digit) = n.div_rem(&radix);
		if digit.bits() > 8 {
			unreachable!()
		}
		if digit > n1 {
			return false;
		}
	}
	true
}

/// Checks if `n` can be written using only zeros and ones,
/// in all bases
/// from the minimun non-trivial base (inclusive)
/// to `max_radix` (exclusive).
pub fn is_0_1_all(n: &UN, max_radix: Non0U8) -> bool {
	// would `rev` be more optimal?
	for radix in MIN_NON_TRIVIAL_BASE.get()..max_radix.get() {
		if !is_0_1(
			n.clone(),
			Non0U8::new(radix).unwrap_or_else(|| unreachable!()),
		) {
			return false;
		}
	}
	true
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn unpacker_works() {
		assert_eq!(
			unpack_as_radix(
				&UN::from(0b1010u8),
				Non0U8::new(3).unwrap_or_else(|| unreachable!("3 == 0 ???"))
			),
			UN::from(27u8 + 3u8)
		);
	}

	#[test]
	fn checker_happy() {
		assert!(is_0_1(
			UN::from(3u8),
			Non0U8::new(3).unwrap_or_else(|| unreachable!())
		));
		assert!(is_0_1(
			UN::from(4u8),
			Non0U8::new(3).unwrap_or_else(|| unreachable!())
		));
		assert!(is_0_1_all(
			&UN::from(4u8),
			Non0U8::new(5).unwrap_or_else(|| unreachable!())
		));
		assert!(is_0_1_all(
			&UN::from(82000u32),
			Non0U8::new(6).unwrap_or_else(|| unreachable!())
		));
		assert!(is_0_1_all(
			&UN::from(82000u32),
			Non0U8::new(6).unwrap_or_else(|| unreachable!())
		));
	}

	#[test]
	fn checker_sad() {
		assert!(!is_0_1(
			UN::from(2u8),
			Non0U8::new(3).unwrap_or_else(|| unreachable!())
		));
		assert!(!is_0_1(
			UN::from(3u8),
			Non0U8::new(4).unwrap_or_else(|| unreachable!())
		));
		assert!(!is_0_1_all(
			&UN::from(3u8),
			Non0U8::new(5).unwrap_or_else(|| unreachable!())
		));
		assert!(!is_0_1_all(
			&UN::from(5u8),
			Non0U8::new(5).unwrap_or_else(|| unreachable!())
		));
		assert!(!is_0_1_all(
			&UN::from(4u8),
			Non0U8::new(6).unwrap_or_else(|| unreachable!())
		));
	}
}
