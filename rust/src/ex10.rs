use primes;

pub fn ex10() {
	println!("{}",
		primes::Primes::new(2_000_000).iter().fold(0u64, |sum, x| sum + x as u64));
}
