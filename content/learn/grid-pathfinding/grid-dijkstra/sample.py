import heapq

DIRS = [(-1, 0), (0, 1), (1, 0), (0, -1)]


def cost(cell):
    return cell if cell >= 2 else 1


def grid_dijkstra(grid, start, goal):
    rows, cols = len(grid), len(grid[0])
    dist = [[None] * cols for _ in range(rows)]
    dist[start[0]][start[1]] = 0
    heap = [(0, start[0], start[1])]
    while heap:
        d, r, c = heapq.heappop(heap)  # @step pop
        if d > dist[r][c]:
            continue  # @step stale
        if (r, c) == goal:
            return [d]  # @step goal
        for dr, dc in DIRS:
            nr, nc = r + dr, c + dc
            if 0 <= nr < rows and 0 <= nc < cols and grid[nr][nc] != 1:
                nd = d + cost(grid[nr][nc])
                if dist[nr][nc] is None or nd < dist[nr][nc]:
                    dist[nr][nc] = nd  # @step relax
                    heapq.heappush(heap, (nd, nr, nc))
    return [-1]
