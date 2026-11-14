def edit_distance(a, b):
    m, n = len(a), len(b)
    cost = [[0] * (n + 1) for _ in range(m + 1)]
    for i in range(m + 1):
        cost[i][0] = i
    for j in range(n + 1):
        cost[0][j] = j
    for i in range(1, m + 1):
        for j in range(1, n + 1):
            if a[i - 1] == b[j - 1]:
                cost[i][j] = cost[i - 1][j - 1]  # @step same
            else:
                cost[i][j] = 1 + min(cost[i - 1][j - 1], cost[i - 1][j], cost[i][j - 1])  # @step change
    return [cost[m][n]]
