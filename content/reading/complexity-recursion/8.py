def f(n):
    if n <= 1:
        return 0
    work = 0
    for i in range(n):
        work += 1
    return work + f(n // 2)
