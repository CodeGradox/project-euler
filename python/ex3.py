n = 600851475143
i = 1

while n != 1:
    i += 2
    while n % i == 0:
        n /= i

print i
