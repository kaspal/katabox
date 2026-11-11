"""Random floor plans; max tests are 200 x 200 (a long snake corridor, and random shelves)."""
import random


def generate(seed, size):
    rng = random.Random(seed)
    if size == "max":
        if seed % 2:
            # Shelf rows with one gap, alternating sides: the route snakes through the
            # floor (199 rows, so the last row, with the goal, is free).
            rows, cols = 199, 200
            grid = [[0] * cols for _ in range(rows)]
            for r in range(1, rows, 2):
                gap = cols - 1 if (r // 2) % 2 == 0 else 0
                grid[r] = [1] * cols
                grid[r][gap] = 0
            return {"grid": grid}
        # Random shelves, plus a carved staircase so the goal is always reachable.
        rows = cols = 200
        grid = [[1 if rng.random() < 0.3 else 0 for _ in range(cols)] for _ in range(rows)]
        r = c = 0
        grid[0][0] = 0
        while (r, c) != (rows - 1, cols - 1):
            if c == cols - 1 or (r < rows - 1 and rng.random() < 0.5):
                r += 1
            else:
                c += 1
            grid[r][c] = 0
        return {"grid": grid}
    rows, cols = rng.randint(1, 4), rng.randint(1, 4)
    density = rng.choice([0.0, 0.2, 0.35, 0.5])
    grid = [[1 if rng.random() < density else 0 for _ in range(cols)] for _ in range(rows)]
    if rng.random() < 0.8:
        grid[0][0] = 0
    if rng.random() < 0.8:
        grid[rows - 1][cols - 1] = 0
    return {"grid": grid}


if __name__ == "__main__":
    import json
    import sys

    print(json.dumps(generate(int(sys.argv[1]) if len(sys.argv) > 1 else 1, sys.argv[2] if len(sys.argv) > 2 else "small")))
