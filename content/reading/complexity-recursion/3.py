def f(n):
    return hanoi(n, 0, 2, 1)

def hanoi(k, src, dst, via):
    if k == 0:
        return 0
    return hanoi(k - 1, src, via, dst) + 1 + hanoi(k - 1, via, dst, src)
