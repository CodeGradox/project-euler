pub fn ex4() -> i32 {
	// 906609
	let mut result = 0;
	for x in (1..999).rev() {
		for y in (1..999).rev() {
			let s = (x*y).to_string();
			let l = &s[0..s.len()/2];
			let r = s.chars()
				.rev()
				.take(s.len()/2)
				.collect::<String>();

			if l == &r[..] && x*y > result {
				result = x*y;
			}
		}
	}
	result
}
