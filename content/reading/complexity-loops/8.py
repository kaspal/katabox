def f(n):
    evens = 0
    for i in range(n):
        if i % 2 == 0:
            evens += 1
    total = 0
    for j in range(2 * n):
        total += j % 3
    return evens + total
