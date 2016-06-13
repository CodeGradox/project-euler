from itertools import takewhile

# Finds all primes below n
def primes(n):
	if n < 2:
		return []

	a = [True] * n

	# loop from 2 to √n
	for i in takewhile(lambda x: x * x < n, xrange(2, n)):
		if a[i]:
			# mark the factors as not prime
			for j in xrange(i * i, n, i):
				a[j] = False

	# return the list of primes, skips 0 and 1
	return [num for num, isprime in enumerate(a[2:], 2) if isprime]

print primes(100)
