def cut_cost(length: int, marks: list[int]) -> int:
    p = [0] + sorted(marks) + [length]
    m = len(p)
    cost = [[0] * m for _ in range(m)]
    for gap in range(2, m):
        for i in range(m - gap):
            j = i + gap
            cost[i][j] = (p[j] - p[i]) + min(cost[i][k] + cost[k][j] for k in range(i + 1, j))
    return cost[0][m - 1]
