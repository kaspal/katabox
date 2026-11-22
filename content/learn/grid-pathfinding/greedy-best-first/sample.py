import heapq

DIRS = [(-1, 0), (0, 1), (1, 0), (0, -1)]


def cost(cell):
    return cell if cell >= 2 else 1


def greedy_best_first(grid, start, goal):
    rows, cols = len(grid), len(grid[0])

    def h(r, c):
        return abs(r - goal[0]) + abs(c - goal[1])

    paid = [[None] * cols for _ in range(rows)]
    paid[start[0]][start[1]] = 0
    heap = [(h(*start), start[0], start[1])]
    while heap:
        _, r, c = heapq.heappop(heap)  # @step pop
        if (r, c) == goal:
            return [paid[r][c]]  # @step goal
        for dr, dc in DIRS:
            nr, nc = r + dr, c + dc
            inside = 0 <= nr < rows and 0 <= nc < cols
            if inside and grid[nr][nc] != 1 and paid[nr][nc] is None:
                paid[nr][nc] = paid[r][c] + cost(grid[nr][nc])  # @step discover
                heapq.heappush(heap, (h(nr, nc), nr, nc))
    return [-1]
