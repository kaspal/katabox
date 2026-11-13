"""Random tree maps and row/column questions as stdin text (a program problem, ADR 0028)."""
import random


def generate(seed, size):
    rng = random.Random(seed)
    if size == "max":
        # The largest map and the most questions; the tree density varies with the seed.
        r, c, q = 300, 300, 20_000
        density = [0.5, 0.1, 0.9][seed % 3]
    elif seed % 5 == 0:
        # Edge shapes: a single row, a single column, one cell; all trees or none.
        r, c = rng.choice([(1, rng.randint(1, 6)), (rng.randint(1, 6), 1), (1, 1), (rng.randint(1, 4), rng.randint(1, 4))])
        q = rng.randint(1, 6)
        density = rng.choice([0.0, 1.0, 0.5])
    else:
        r, c, q = rng.randint(1, 5), rng.randint(1, 5), rng.randint(1, 8)
        density = rng.random()
    grid = ["".join("#" if rng.random() < density else "." for _ in range(c)) for _ in range(r)]
    queries = []
    for _ in range(q):
        if rng.random() < 0.5:
            queries.append(f"row {rng.randrange(r)}")
        else:
            queries.append(f"col {rng.randrange(c)}")
    lines = [f"{r} {c}", *grid, str(q), *queries]
    return "\n".join(lines) + "\n"


if __name__ == "__main__":
    import sys

    args = sys.argv[1:]
    sys.stdout.write(generate(int(args[0]) if args else 1, args[1] if len(args) > 1 else "small"))
