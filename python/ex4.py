r = 0

for x in xrange(1, 1000):
    for y in xrange(x, 1000):
        s = str(x * y)
        if s == s[::-1] and x * y > r:
            r = x * y

print r
