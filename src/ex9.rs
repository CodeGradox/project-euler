// 31875000

pub fn ex9() -> i32 {
	for a in 1..1000 {
		for b in (a+1)..(1000 - a) {
			let c = 1000 - a - b;
			if a*a + b*b == c*c {
				return a*b*c;
			}
		}
	}
	0
}
