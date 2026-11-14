def lcs(a, b):
    m, n = len(a), len(b)
    best = [[0] * (n + 1) for _ in range(m + 1)]
    for i in range(1, m + 1):
        for j in range(1, n + 1):
            if a[i - 1] == b[j - 1]:
                best[i][j] = best[i - 1][j - 1] + 1  # @step match
            else:
                best[i][j] = max(best[i - 1][j], best[i][j - 1])  # @step carry
    out, i, j = [], m, n
    while i > 0 and j > 0:
        if a[i - 1] == b[j - 1]:
            out.append(a[i - 1])  # @step trace
            i, j = i - 1, j - 1
        elif best[i - 1][j] >= best[i][j - 1]:
            i -= 1
        else:
            j -= 1
    return [len(out)] + [ord(c) - 97 for c in reversed(out)]
