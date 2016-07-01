pub fn ex6() {
	let n = 100u64;
	let sum_n = (n * (n + 1) * (2 * n + 1)) / 6;
	let prod_n = n * (n + 1) / 2;
	println!("{:?}", prod_n * prod_n - sum_n);
}
