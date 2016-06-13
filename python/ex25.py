a = 1
b = 1
idx = 2

while b < 10**999:
	a, b = b, a + b
	idx += 1

print idx
