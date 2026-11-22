def f(n):
    power = 1
    steps = 0
    while power <= n:
        power *= 3
        steps += 1
    return steps
