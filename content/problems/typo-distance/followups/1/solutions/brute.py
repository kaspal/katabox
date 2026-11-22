def within_fixes(typed: str, meant: str, k: int) -> bool:
    # Obviously correct: the full edit-distance table.
    n, m = len(typed), len(meant)
    d = [[0] * (m + 1) for _ in range(n + 1)]
    for i in range(n + 1):
        for j in range(m + 1):
            if i == 0 or j == 0:
                d[i][j] = i + j
            else:
                d[i][j] = min(d[i - 1][j] + 1, d[i][j - 1] + 1, d[i - 1][j - 1] + (typed[i - 1] != meant[j - 1]))
    return d[n][m] <= k
