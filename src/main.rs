mod util;
#[allow(clippy::wildcard_imports)]
use util::*;

/// lowest unknown, as of year 2024
const BASE: Non0U8 = match Non0U8::new(6) {
	Some(n) => n,
	_ => unreachable!(),
};

/// According to [Numberphile](https://youtu.be/LNS1fabDkeA)
/// thousand-digit numerals have already been checked.
const START: u128 = u128::MAX;

fn main() {
	let start = START;
	// prevent us from using a trivial value
	assert!(start > 1);

	// each bit represents a digit in radix `BASE`
	let mut packed_numeral = BitVec::new(UN::from(start));
	loop {
		/*
		We must pay the price of conversion,
		regardless of representation.
		Maybe someone can come up with a clever algorithm
		that exploits previously-unpacked values to infer the next?
		*/
		let n = unpack_as_radix(packed_numeral.clone(), BASE);
		// by definition, `BASE` is already checked,
		// so no need to include it in the range
		if is_0_1_all(&n, BASE) {
			println!("{:#x}", *packed_numeral);
			break;
		}
		// skip all `n` that match `!is_0_1(n, BASE)`
		packed_numeral.inc();
	}
}
