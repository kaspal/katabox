MOD = 1_000_003


def _mul(a: list[list[int]], b: list[list[int]]) -> list[list[int]]:
    return [[sum(a[i][k] * b[k][j] for k in range(3)) % MOD for j in range(3)] for i in range(3)]


def tilings_long(n: int) -> int:
    # (f(k), f(k-1), f(k-2)) -> (f(k+1), f(k), f(k-1)) with f(k+1) = f(k) + f(k-1) + f(k-2).
    step = [[1, 1, 1], [1, 0, 0], [0, 1, 0]]
    power = [[1, 0, 0], [0, 1, 0], [0, 0, 1]]
    e = n
    while e:
        if e & 1:
            power = _mul(power, step)
        step = _mul(step, step)
        e >>= 1
    # Start from (f(0), f(-1), f(-2)) = (1, 0, 0).
    return power[0][0]
