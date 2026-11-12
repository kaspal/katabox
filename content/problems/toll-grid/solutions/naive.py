def cheapest_crossing(tolls: list[list[int]]) -> int:
    # Correct but slow: sweep every state in row order, relaxing all neighbours,
    # until a whole sweep changes nothing (Bellman–Ford style). A winding cheapest
    # route needs a sweep for almost every step it takes against the sweep order.
    rows, cols = len(tolls), len(tolls[0])
    inf = float("inf")
    cost = [[[inf] * cols for _ in range(rows)] for _ in range(2)]
    cost[0][0][0] = tolls[0][0]
    cost[1][0][0] = tolls[0][0] // 2
    changed = True
    while changed:
        changed = False
        for used in (0, 1):
            for r in range(rows):
                for c in range(cols):
                    d = cost[used][r][c]
                    if d == inf:
                        continue
                    for nr, nc in ((r - 1, c), (r + 1, c), (r, c - 1), (r, c + 1)):
                        if 0 <= nr < rows and 0 <= nc < cols:
                            toll = tolls[nr][nc]
                            if d + toll < cost[used][nr][nc]:
                                cost[used][nr][nc] = d + toll
                                changed = True
                            if not used and d + toll // 2 < cost[1][nr][nc]:
                                cost[1][nr][nc] = d + toll // 2
                                changed = True
    return min(cost[0][rows - 1][cols - 1], cost[1][rows - 1][cols - 1])
