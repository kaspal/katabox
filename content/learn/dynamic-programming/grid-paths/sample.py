def grid_paths(rows, cols, cells):
    ways = [[0] * cols for _ in range(rows)]
    for r in range(rows):
        for c in range(cols):
            if cells[r * cols + c] == 1:
                ways[r][c] = 0  # @step blocked
            elif r == 0 and c == 0:
                ways[r][c] = 1
            else:
                up = ways[r - 1][c] if r > 0 else 0
                left = ways[r][c - 1] if c > 0 else 0
                ways[r][c] = up + left  # @step add
    return [ways[rows - 1][cols - 1]]
