pub fn ex3() -> u64 {
	let mut n = 600851475143;
	let mut i = 1;

	while n != 1 {
		i += 2;
		while n % i == 0 {
			n /= i;
		}
	}
	i
}
