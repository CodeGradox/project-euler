// 4613732

pub fn ex2() {
	let (a, b, mut sum) = (&mut 1, &mut 1, 0);

	while *b < 4_000_000 {
		*b += *a;
		super::std::mem::swap(a, b);
		if *b % 2 == 0 {
			sum += *b;
		}
	}
	println!("{}", sum);
}
