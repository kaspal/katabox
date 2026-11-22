def f(n):
    work = 0
    width = n
    while width > 1:
        for i in range(n):
            work += 1
        width //= 2
    return work
