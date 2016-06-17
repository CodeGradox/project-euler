extern crate num;

use num::bigint::ToBigInt;
use std::mem::swap;

pub fn ex25() {
	let mut a = 1.to_bigint().unwrap();
	let mut b = 1.to_bigint().unwrap();
	let mut idx = 1;
	let threshold = num::pow(10.to_bigint().unwrap(), 999);

	while b < threshold {
		b = &a + &b;
		swap(&mut a, &mut b);
		idx += 1;
	}

	println!("{}", idx);
}
