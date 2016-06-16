/*
 * This problem can be easly solved by typign in "40 chose 20" in google
 */

pub fn ex15() {
	let s = 21; // 20x20 grid
	let mut v: Vec<Vec<u64>> = vec![vec![1;s]; s];

	for i in 1..s {
		for j in 1..s {
			v[i][j] = v[i-1][j] + v[i][j-1];
		}
	}
	println!("{}", v[s-1][s-1]);
}
