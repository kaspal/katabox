def gentlest_climb(heights: list[list[int]]) -> int:
    # Obviously correct: for each candidate limit (0 and every neighbour difference),
    # smallest first, check with a plain search whether the end is reachable.
    rows, cols = len(heights), len(heights[0])
    diffs = {0}
    for r in range(rows):
        for c in range(cols):
            if r + 1 < rows:
                diffs.add(abs(heights[r][c] - heights[r + 1][c]))
            if c + 1 < cols:
                diffs.add(abs(heights[r][c] - heights[r][c + 1]))
    for limit in sorted(diffs):
        seen = {(0, 0)}
        stack = [(0, 0)]
        while stack:
            r, c = stack.pop()
            for nr, nc in ((r - 1, c), (r + 1, c), (r, c - 1), (r, c + 1)):
                if 0 <= nr < rows and 0 <= nc < cols and (nr, nc) not in seen:
                    if abs(heights[nr][nc] - heights[r][c]) <= limit:
                        seen.add((nr, nc))
                        stack.append((nr, nc))
        if (rows - 1, cols - 1) in seen:
            return limit
    return -1
