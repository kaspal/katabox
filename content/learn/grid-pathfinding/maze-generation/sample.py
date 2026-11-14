DIRS = [(-1, 0), (0, 1), (1, 0), (0, -1)]


def make_maze(k, seed):
    size = 2 * k + 1
    grid = [[1] * size for _ in range(size)]
    state = seed

    def rand():
        nonlocal state
        state = state * 48271 % 2147483647
        return state

    grid[1][1] = 0
    stack = [(1, 1)]
    while stack:
        r, c = stack[-1]
        options = []
        for dr, dc in DIRS:
            nr, nc = r + 2 * dr, c + 2 * dc
            if 0 < nr < size and 0 < nc < size and grid[nr][nc] == 1:
                options.append((dr, dc))
        if options:
            dr, dc = options[rand() % len(options)]
            grid[r + dr][c + dc] = 0
            grid[r + 2 * dr][c + 2 * dc] = 0  # @step carve
            stack.append((r + 2 * dr, c + 2 * dc))
        else:
            stack.pop()  # @step back
    return [cell for row in grid for cell in row]
