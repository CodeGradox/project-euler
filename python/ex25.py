a = b = 1
idx = 2
threshold = 10**999

while b < threshold:
	a, b = b, a + b
	idx += 1

print idx
