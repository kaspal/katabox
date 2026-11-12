def typo_distance(typed: str, meant: str) -> int:
    m = len(meant)
    prev = list(range(m + 1))
    for i, x in enumerate(typed, 1):
        cur = [i] + [0] * m
        for j, y in enumerate(meant, 1):
            if x == y:
                cur[j] = prev[j - 1]
            else:
                a, b, c = prev[j], cur[j - 1], prev[j - 1]
                cur[j] = 1 + (a if a < b and a < c else b if b < c else c)
        prev = cur
    return prev[m]
