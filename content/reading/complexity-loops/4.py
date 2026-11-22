def f(n):
    pairs = 0
    for i in range(n):
        for j in range(i + 1, n):
            if (i + j) % 3 == 0:
                pairs += 1
    return pairs
