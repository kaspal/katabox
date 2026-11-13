P = 1_000_003


def blocked_walks(rows: int, cols: int, blocked: list[list[int]]) -> int:
    top = rows + cols
    fact = [1] * (top + 1)
    for i in range(1, top + 1):
        fact[i] = fact[i - 1] * i % P
    inv = [1] * (top + 1)
    inv[top] = pow(fact[top], P - 2, P)
    for i in range(top, 0, -1):
        inv[i - 1] = inv[i] * i % P

    def paths(dr, dc):
        return fact[dr + dc] * inv[dr] % P * inv[dc] % P

    points = sorted(map(tuple, blocked)) + [(rows, cols)]
    good = []
    for i, (r, c) in enumerate(points):
        ways = paths(r, c)
        for j in range(i):
            pr, pc = points[j]
            if pr <= r and pc <= c:
                ways -= good[j] * paths(r - pr, c - pc)
        good.append(ways % P)
    return good[-1]
