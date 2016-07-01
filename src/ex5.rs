pub fn ex5() {
	for i in (1..).map(|x| x * 2520) {
		if (11..21).all(|j| i % j == 0) {
			println!("{}", i);
			break;
		}
	}
}
