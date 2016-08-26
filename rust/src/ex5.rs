pub fn ex5() {
	for i in (1..).map(|x| x * 2520) {
		if (11..21).all(|j| i % j == 0) {
			println!("{}", i);
			break;
		}
	}
}

pub fn ex5_f() {
	let val = (1..)
		.map(|x| x * 2520)
		.skip_while(|x| !(11..21).all(|y| x % y == 0))
		.nth(0);
	println!("{:?}", val);
}
