a, b, s = 1, 1, 0

while b < 4000000:
	b = a + b
	a, b = b, a
	if b % 2 == 0:
		s += b

print s
