// https://en.wikipedia.org/wiki/Sieve_of_Eratosthenes
pub fn primes(n: usize) -> Vec<usize> {
	if n < 2 {
		return vec![];
	}
	let mut a = vec![true; n];

	for i in 2..((n as f64).sqrt().ceil() as usize) {
		if a[i] {
			let mut j = i * i;
			let mut cnt = 1;
			while j < n {
				a[j] = false;
				j = i*i + i*cnt;
				cnt += 1;
			}
		}
	}
	(a[2..]).iter().zip(2..).filter(|&(&a, _)| a).map(|(_, b)| b).collect::<Vec<usize>>()
}
