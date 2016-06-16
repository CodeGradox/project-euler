pub fn ex3() {
	let mut n: u64 = 600_851_475_143;
	let mut i = 1;

	while n != 1 {
		i += 2;
		while n % i == 0 {
			n /= i;
		}
	}
	println!("{}", i);
}
