def f(n):
    a = list(range(0, 2 * n, 2))
    b = list(range(0, 3 * n, 3))
    common = 0
    for x in a:
        j = 0
        while j < len(b) and b[j] != x:
            j += 1
        if j < len(b):
            common += 1
    return common
