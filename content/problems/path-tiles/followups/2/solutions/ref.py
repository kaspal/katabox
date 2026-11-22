MOD = 1_000_000_007


def tilings_apart(n: int) -> int:
    # a[i]: pavings of length i not ending in a long slab (the empty one included);
    # b[i]: pavings of length i ending in a long slab.
    a = [0] * (n + 1)
    b = [0] * (n + 1)
    a[0] = 1
    for i in range(1, n + 1):
        a[i] = a[i - 1] + b[i - 1]
        if i >= 2:
            a[i] += a[i - 2] + b[i - 2]
        if i >= 3:
            b[i] = a[i - 3]
        a[i] %= MOD
    return (a[n] + b[n]) % MOD
