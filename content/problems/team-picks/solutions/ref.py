P = 1_000_003


def team_picks(queries: list[list[int]]) -> list[int]:
    top = max(n for n, _ in queries)
    fact = [1] * (top + 1)
    for i in range(1, top + 1):
        fact[i] = fact[i - 1] * i % P
    inv = [1] * (top + 1)
    inv[top] = pow(fact[top], P - 2, P)
    for i in range(top, 0, -1):
        inv[i - 1] = inv[i] * i % P
    return [fact[n] * inv[k] % P * inv[n - k] % P if k <= n else 0 for n, k in queries]
