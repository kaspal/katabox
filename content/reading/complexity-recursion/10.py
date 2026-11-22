def f(n):
    return subsets(0, 0, n)

def subsets(i, total, n):
    if i == n:
        return 1 if total % 3 == 0 else 0
    return subsets(i + 1, total + i, n) + subsets(i + 1, total, n)
