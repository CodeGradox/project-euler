pub fn ex5() {
	let mut t = 2520;
	loop {
		for i in 11..21 {
			if t%i != 0 {
				break;
			}
			if i == 20 {
				println!("{}", t);
				return;
			}
		}
		t += 2520;
	}
}
