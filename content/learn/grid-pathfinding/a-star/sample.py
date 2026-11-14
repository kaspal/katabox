import heapq

DIRS = [(-1, 0), (0, 1), (1, 0), (0, -1)]


def cost(cell):
    return cell if cell >= 2 else 1


def a_star(grid, start, goal):
    rows, cols = len(grid), len(grid[0])

    def h(r, c):
        return abs(r - goal[0]) + abs(c - goal[1])

    g = [[None] * cols for _ in range(rows)]
    g[start[0]][start[1]] = 0
    heap = [(h(*start), h(*start), start[0], start[1])]
    while heap:
        f, hr, r, c = heapq.heappop(heap)  # @step pop
        if f - hr > g[r][c]:
            continue  # @step stale
        if (r, c) == goal:
            return [g[r][c]]  # @step goal
        for dr, dc in DIRS:
            nr, nc = r + dr, c + dc
            if 0 <= nr < rows and 0 <= nc < cols and grid[nr][nc] != 1:
                ng = g[r][c] + cost(grid[nr][nc])
                if g[nr][nc] is None or ng < g[nr][nc]:
                    g[nr][nc] = ng  # @step relax
                    heapq.heappush(heap, (ng + h(nr, nc), h(nr, nc), nr, nc))
    return [-1]
