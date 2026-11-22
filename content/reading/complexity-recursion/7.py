def f(n):
    if n == 0:
        return 0
    total = 0
    for i in range(n):
        total += i
    return total + f(n - 1)
