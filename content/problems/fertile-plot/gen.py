"""Field surveys as stdin text (a program problem, ADR 0028)."""
import random


def generate(seed, size):
    rng = random.Random(seed)
    if size == "max":
        # About 10⁵ cells (about 260 KB); trying every rectangle is ~2.5·10⁹ steps.
        if seed % 3 == 0:
            # A near-square field: the reference's worst case, ~1.6·10⁷ Kadane steps.
            rows, cols = (315, 317) if seed % 2 else (317, 315)
        else:
            # A long thin field (wide on odd seeds, tall on even ones): ~5·10⁵ Kadane steps
            # when the short side is in the outer loops.
            short = rng.choice([8, 10, 16, 20])
            long_side = 100_000 // short
            rows, cols = (short, long_side) if seed % 2 else (long_side, short)

        def score():
            # Mostly small scores (keeps the text short), with rare rich spots and rocks.
            return rng.randint(-1000, 1000) if rng.random() < 0.03 else rng.randint(-9, 9)

        grid = [[score() for _ in range(cols)] for _ in range(rows)]
    elif seed % 5 == 0:
        # Edge shapes: a single cell, all negative, all extreme, zeros, one row or column.
        rows, cols = rng.choice([(1, 1), (1, rng.randint(2, 6)), (rng.randint(2, 6), 1), (rng.randint(1, 4), rng.randint(1, 4))])
        kind = rng.randrange(4)
        if kind == 0:
            grid = [[rng.randint(-1000, -1) for _ in range(cols)] for _ in range(rows)]
        elif kind == 1:
            grid = [[rng.choice([-1000, 1000]) for _ in range(cols)] for _ in range(rows)]
        elif kind == 2:
            grid = [[rng.choice([0, 0, -1]) for _ in range(cols)] for _ in range(rows)]
        else:
            grid = [[1000] * cols for _ in range(rows)]
    else:
        rows, cols = rng.randint(1, 5), rng.randint(1, 5)
        grid = [[rng.randint(-10, 10) for _ in range(cols)] for _ in range(rows)]
    lines = [f"{rows} {cols}"] + [" ".join(map(str, row)) for row in grid]
    return "\n".join(lines) + "\n"


if __name__ == "__main__":
    import sys

    args = sys.argv[1:]
    sys.stdout.write(generate(int(args[0]) if args else 1, args[1] if len(args) > 1 else "small"))
