def gentlest_climb(heights: list[list[int]]) -> int:
    # Correct but slow: try every distinct step difficulty in increasing order, with a
    # fresh search of the whole grid for each one.
    rows, cols = len(heights), len(heights[0])
    diffs = {0}
    for r in range(rows):
        for c in range(cols):
            if r + 1 < rows:
                diffs.add(abs(heights[r][c] - heights[r + 1][c]))
            if c + 1 < cols:
                diffs.add(abs(heights[r][c] - heights[r][c + 1]))
    for limit in sorted(diffs):
        seen = [[False] * cols for _ in range(rows)]
        seen[0][0] = True
        stack = [(0, 0)]
        while stack:
            r, c = stack.pop()
            for nr, nc in ((r - 1, c), (r + 1, c), (r, c - 1), (r, c + 1)):
                if 0 <= nr < rows and 0 <= nc < cols and not seen[nr][nc]:
                    if abs(heights[nr][nc] - heights[r][c]) <= limit:
                        seen[nr][nc] = True
                        stack.append((nr, nc))
        if seen[rows - 1][cols - 1]:
            return limit
    return -1
