"""Wall maps of free and blocked squares as stdin text (a program problem, ADR 0028)."""
import random


def render(rows):
    return "\n".join([f"{len(rows)} {len(rows[0])}", *rows]) + "\n"


def wall(rng, r, c, blocked):
    return ["".join("#" if rng.random() < blocked else "." for _ in range(c)) for _ in range(r)]


def generate(seed, size):
    rng = random.Random(seed)
    if size == "max":
        if seed % 2 == 1:
            # 1500 × 1500 with only 40 blocked squares: almost every row is one long free
            # run, so trying every left edge and width row by row is about 1.7·10⁹ steps.
            rows = [["."] * 1500 for _ in range(1500)]
            for _ in range(40):
                rows[rng.randrange(1500)][rng.randrange(1500)] = "#"
            return render(["".join(row) for row in rows])
        # 1000 × 1000 with a free band and dense blocks elsewhere.
        rows = wall(rng, 1000, 1000, 0.3)
        top = rng.randint(100, 800)
        for y in range(top, top + 150):
            rows[y] = "".join("." if 200 <= x < 900 or ch == "." else ch for x, ch in enumerate(rows[y]))
        return render(rows)
    if seed % 5 == 0:
        # Edge shapes: one square, all blocked, all free, one row, one column.
        kind = rng.randrange(5)
        if kind == 0:
            return render([rng.choice(".#")])
        if kind == 1:
            return render(["#" * rng.randint(1, 6)] * rng.randint(1, 6))
        if kind == 2:
            return render(["." * rng.randint(1, 6)] * rng.randint(1, 6))
        if kind == 3:
            return render(wall(rng, 1, rng.randint(2, 10), 0.3))
        return render(wall(rng, rng.randint(2, 10), 1, 0.3))
    return render(wall(rng, rng.randint(1, 7), rng.randint(1, 7), rng.choice([0.15, 0.3, 0.5])))


if __name__ == "__main__":
    import sys

    args = sys.argv[1:]
    sys.stdout.write(generate(int(args[0]) if args else 1, args[1] if len(args) > 1 else "small"))
