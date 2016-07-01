use std::mem::swap;

pub fn ex2() {
	let mut a = 1;
	let mut b = 1;
	let mut sum = 0;

	while b < 4_000_000 {
		a += b;
		swap(&mut a, &mut b);
		if b % 2 == 0 {
			sum += b;
		}
	}
	println!("{}", sum);
}
