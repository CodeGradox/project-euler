# s = [i * i for i in xrange(1, 11)]
n = 100
sum_n = (n * (n + 1) * (2 * n + 1)) / 6
prod_n = n * (n + 1) / 2
print prod_n * prod_n - sum_n
