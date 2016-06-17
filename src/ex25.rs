extern crate num;

use num::bigint::ToBigInt;

pub fn ex25() {
	let mut a = &mut 1.to_bigint().unwrap();
	let mut b = &mut 1.to_bigint().unwrap();
	let mut idx = 2;

	while *b < num::pow(10.to_bigint().unwrap(), 999) {
		*b = &*a + &*b;
		super::std::mem::swap(a, b);
		idx += 1;
	}

	println!("{}", idx);
}
