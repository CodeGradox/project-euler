use primes;

pub fn ex10() -> u64 {
	primes::Primes::new(2_000_000).iter().fold(0, |sum, x| sum + x) as u64
}
