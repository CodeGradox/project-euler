// https://en.wikipedia.org/wiki/Sieve_of_Eratosthenes
pub fn primes(n: usize) -> Vec<usize> {
	if n < 2 {
		return vec![];
	}
	let mut a = vec![true; n];

	for i in (2..n).take_while(|x| x * x < n) {
		if a[i] {
			let mut j = i * i;
			while j < n {
				a[j] = false;
				j += i;
			}
		}
	}
	a[2..]
		.iter()
		.zip(2..)
		.filter_map(|(isprime, num)| if *isprime { Some(num) } else { None })
		.collect::<Vec<usize>>()
}
