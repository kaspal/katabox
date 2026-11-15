"""A sea map and land reclaimed square by square, as stdin text (a program problem, ADR 0028)."""
import random


def render(r, c, cells):
    return "\n".join([f"{r} {c} {len(cells)}"] + [f"{y} {x}" for y, x in cells]) + "\n"


def sized(rng, n):
    """Complexity profile input (ADR 0024): n new squares on a 400 × 400 sea."""
    cells = rng.sample([(y, x) for y in range(1, 401) for x in range(1, 401)], n)
    return render(400, 400, cells)


def generate(seed, size):
    rng = random.Random(seed)
    if isinstance(size, int):
        return sized(rng, size)
    if size == "max":
        if seed % 2 == 1:
            # 5·10⁴ new squares scattered over a 1000 × 1000 sea: recounting every island
            # after each one is about 10⁹ square visits.
            return render(1000, 1000, [(rng.randint(1, 1000), rng.randint(1, 1000)) for _ in range(50_000)])
        # A 200 × 200 sea filled almost completely (many merges), with repeated squares.
        cells = [(y, x) for y in range(1, 201) for x in range(1, 201)]
        rng.shuffle(cells)
        cells = cells[:30_000]
        cells += rng.sample(cells, 2_000)
        rng.shuffle(cells)
        return render(200, 200, cells)
    if seed % 5 == 0:
        # Edge shapes: a single square, the same square again, a diagonal (no merges),
        # a ring that closes, one row.
        kind = rng.randrange(4)
        if kind == 0:
            return render(1, 1, [(1, 1)] * rng.randint(1, 3))
        if kind == 1:
            return render(4, 4, [(i, i) for i in range(1, 5)])
        if kind == 2:
            return render(3, 3, [(1, 1), (1, 3), (3, 1), (3, 3), (1, 2), (2, 1), (3, 2), (2, 3), (2, 2)])
        return render(1, 9, [(1, x) for x in rng.sample(range(1, 10), 9)])
    r, c = rng.randint(1, 5), rng.randint(1, 5)
    return render(r, c, [(rng.randint(1, r), rng.randint(1, c)) for _ in range(rng.randint(1, 10))])


if __name__ == "__main__":
    import sys

    args = sys.argv[1:]
    sys.stdout.write(generate(int(args[0]) if args else 1, args[1] if len(args) > 1 else "small"))
