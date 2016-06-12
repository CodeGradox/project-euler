from itertools import takewhile

# Finds all primes below n
def primes(n):
	if n < 2:
		return []

	a = [True for i in xrange(0, n)]

	for i in takewhile(lambda x: x * x < n, xrange(2, n)):
		if a[i]:
			for j in xrange(i * i, n, i):
				a[j] = False

	return [d for d, i in enumerate(a[2:], 2) if i]

print primes(100)
