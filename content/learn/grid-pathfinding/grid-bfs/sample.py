from collections import deque

DIRS = [(-1, 0), (0, 1), (1, 0), (0, -1)]


def grid_bfs(grid, start, goal):
    rows, cols = len(grid), len(grid[0])
    dist = [[-1] * cols for _ in range(rows)]
    dist[start[0]][start[1]] = 0
    queue = deque([start])
    while queue:
        r, c = queue.popleft()  # @step visit
        if (r, c) == goal:
            return [dist[r][c]]  # @step goal
        for dr, dc in DIRS:
            nr, nc = r + dr, c + dc
            inside = 0 <= nr < rows and 0 <= nc < cols
            if inside and grid[nr][nc] != 1 and dist[nr][nc] == -1:
                dist[nr][nc] = dist[r][c] + 1  # @step discover
                queue.append((nr, nc))
    return [-1]
