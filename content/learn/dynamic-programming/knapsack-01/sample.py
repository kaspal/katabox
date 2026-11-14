def knapsack(capacity, items):
    n = len(items)
    best = [[0] * (capacity + 1) for _ in range(n + 1)]
    for i in range(1, n + 1):
        weight, value = items[i - 1]
        for c in range(capacity + 1):
            best[i][c] = best[i - 1][c]  # @step skip
            if weight <= c and best[i - 1][c - weight] + value > best[i][c]:
                best[i][c] = best[i - 1][c - weight] + value  # @step take
    chosen, c = [], capacity
    for i in range(n, 0, -1):
        if best[i][c] != best[i - 1][c]:
            chosen.append(i - 1)  # @step trace
            c -= items[i - 1][0]
    return [best[n][capacity]] + chosen[::-1]
