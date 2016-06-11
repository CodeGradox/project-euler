use primes;

pub fn ex10() -> u64 {
	primes::primes(2000000).iter().fold(0u64, |sum, x| sum + (*x as u64))
}
