def f(n):
    return power(3, n, 1_000_007)

def power(base, e, mod):
    if e == 0:
        return 1
    half = power(base, e // 2, mod)
    result = half * half % mod
    if e % 2 == 1:
        result = result * base % mod
    return result
