a = b = idx = 1

while b < 10**999:
	b = a + b
	a, b = b, a
	idx += 1

print idx
