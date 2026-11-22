def matrix_chain(dims):
    n = len(dims) - 1
    cost = [[0] * n for _ in range(n)]
    for span in range(2, n + 1):
        for i in range(n - span + 1):
            j = i + span - 1
            cost[i][j] = 10 ** 9
            for k in range(i, j):
                total = cost[i][k] + cost[k + 1][j]
                total += dims[i] * dims[k + 1] * dims[j + 1]
                if total < cost[i][j]:
                    cost[i][j] = total  # @step split
    return [cost[0][n - 1]]
