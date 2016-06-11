pub fn ex14() -> u64 {
	let mut largest = (1, 1); // n, cnt

	for i in 1..1000000 {
		let mut n = i;
		let mut cnt = 1;

		while n != 1 {
			if n % 2 == 0 {
				n /= 2;
			} else {
				n = 3*n + 1;
			}
			cnt += 1;
		}

		if cnt > largest.1 {
			largest.0 = i;
			largest.1 = cnt;
		}
	}
	largest.0
}
