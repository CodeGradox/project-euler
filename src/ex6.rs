pub fn ex6() -> u32 {
	let n = 100;
	let sum_n = (n * (n + 1) * (2 * n + 1)) / 6;
	let prod_n = n * (n + 1) / 2;
	prod_n * prod_n - sum_n
}
