extern crate num;

use num::bigint::ToBigInt;

pub fn ex20() {
	let s = (1..100)
		.fold(100.to_bigint().unwrap(), |prod, num| prod * num.to_bigint().unwrap())
		.to_str_radix(10)
		.chars()
		.map(|x| x.to_digit(10).unwrap() as u64)
		.fold(0u64, |sum, num| sum + num);

	println!("{}", s);
}
