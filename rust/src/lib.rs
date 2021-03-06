#![feature(test)]
extern crate test;
extern crate num;
extern crate sieve_of_eratosthenes;

pub mod ex1;
pub mod ex2;
pub mod ex3;
pub mod ex4;
pub mod ex5;
pub mod ex8;
pub mod ex9;
pub mod ex10;
pub mod ex11;
pub mod ex12;
pub mod ex13;
pub mod ex14;
pub mod ex15;
pub mod ex16;
pub mod ex20;
pub mod ex25;

#[cfg(test)]
mod tests {
	use test::Bencher;
	use super::*;

	#[bench]
	fn bench_primes_bit(b: &mut Bencher) {
		b.iter(|| primes::Primes::new(75000));
	}
}
