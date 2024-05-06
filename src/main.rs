mod util;
#[allow(clippy::wildcard_imports)]
use util::*;

/// lowest unknown, as of year 2024
const BASE: Non0U8 = match Non0U8::new(6) {
	Some(n) => n,
	_ => unreachable!(),
};

/// thousand-digit numerals have already been checked.
/// source: `README.md` links.
const START_LEN: u16 = 0x1000;

fn main() {
	println!(
		"{:#x}",
		// each bit represents a digit in radix `BASE`
		successors(Some(BitVec::new(UN::one() << START_LEN)), |n| Some(
			BitVec::new((**n).clone() + 1u8)
		))
		/*
		We must pay the price of conversion,
		regardless of representation.
		Maybe someone can come up with a clever algorithm
		that exploits previously-unpacked values to infer the next?
		*/
		.map(|packed| unpack_as_radix(packed, BASE))
		// by definition, `BASE` is already checked,
		// so no need to include it in the range
		.find(|n| is_0_1_all(n, BASE))
		.unwrap_or_else(|| unreachable!())
	);
}
