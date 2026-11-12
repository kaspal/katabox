MOD = 1_000_000_007


def plantings(length: int, widths: list[int]) -> int:
    k = len(widths)
    ways = [[0] * k for _ in range(length + 1)]
    total = [0] * (length + 1)
    total[0] = 1
    for x in range(1, length + 1):
        row = ways[x]
        for kind, w in enumerate(widths):
            if w <= x:
                row[kind] = (total[x - w] - ways[x - w][kind]) % MOD
        total[x] = sum(row) % MOD
    return total[length]
