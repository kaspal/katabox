def f(n):
    total = 0
    for k in range(1, 101):
        total += n % k
    return total
