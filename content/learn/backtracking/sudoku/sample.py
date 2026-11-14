def solve_sudoku(size, cells):
    box = round(size ** 0.5)
    grid = list(cells)

    def ok(at, d):
        r, c = divmod(at, size)
        for k in range(size):
            if grid[r * size + k] == d or grid[k * size + c] == d:
                return False
        top, left = r - r % box, c - c % box
        for rr in range(top, top + box):
            for cc in range(left, left + box):
                if grid[rr * size + cc] == d:
                    return False
        return True

    def fill():
        if 0 not in grid:
            return True
        at = grid.index(0)
        for d in range(1, size + 1):
            if ok(at, d):
                grid[at] = d  # @step place
                if fill():
                    return True
                grid[at] = 0  # @step undo
        return False

    return grid if fill() else [-1]
