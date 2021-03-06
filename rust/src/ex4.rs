pub fn ex4() {
	let mut result = 0u64;
	for x in 100..1000 {
		for y in x..1000 {
			let s = (x * y).to_string();
			let l = s.chars().take(s.len() / 2);
			let r = s.chars().rev().take(s.len() / 2);

			if l.zip(r).all(|(a, b)| a == b) && x * y > result {
				result = x * y;
			}
		}
	}
	println!("{}", result);
}
