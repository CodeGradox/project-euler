// 233168

pub fn ex1() {
	println!("{}",
		(3..1000).filter(|x| x % 3 == 0 || x % 5 == 0).fold(0, |sum, x| sum + x));
}