pub fn ex12() -> u64 {
	let mut c = 1;
	loop {
		let n = c*(c+1)/2;
		let d = divisors(n);
		println!("{} {}", n, d);
		if d > 4 {
			return n;
		}
		c += 1;
	}
}

fn divisors(x: u64) -> u64 {
	let mut limit = x;
	let mut nod = 0;

	if x == 1 { return 1; }

	for i in 1..limit {
		if x % i == 0 {
			limit = x / i;
			if limit != i {
				nod += 1;
			}
			nod += 1;
		}
	}
	nod
}
