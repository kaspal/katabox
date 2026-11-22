def f(n):
    count = 0
    for i in range(n):
        for j in range(i):
            count += 1
    return count
