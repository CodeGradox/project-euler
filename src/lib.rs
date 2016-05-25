#![feature(test)]
extern crate test;
pub mod ex1;
pub mod ex2;
pub mod ex4;
pub mod ex5;
pub mod ex8;
pub mod ex9;
pub mod ex10;

#[cfg(test)]
mod tests {
	use test::Bencher;
	use super::*;

	#[bench]
	fn bench(b: &mut Bencher) {
		b.iter(|| ex10::ex10());
	}
}
