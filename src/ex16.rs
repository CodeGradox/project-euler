extern crate num;

use num::bigint::{ToBigInt};

pub fn ex16() {
	println!("{}",
		num::pow(ToBigInt::to_bigint(&2).unwrap(), 1000)
		.to_str_radix(10)
		.chars()
		.map(|x| x.to_digit(10).unwrap() as u64)
		.fold(0u64, |sum, num| sum + num));
}
