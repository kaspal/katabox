def f(n):
    if n <= 1:
        return 0
    return 1 + f(n // 2)
