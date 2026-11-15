"""Trail-camera species logs and range questions as stdin text (a program problem, ADR 0028)."""
import random


def render(ids, questions):
    lines = [str(len(ids)), " ".join(map(str, ids)), str(len(questions))]
    lines += [f"{l} {r}" for l, r in questions]
    return "\n".join(lines) + "\n"


def wide(rng, n, q):
    """q questions each covering most of the log."""
    out = []
    for _ in range(q):
        width = rng.randint(n * 6 // 10, n)
        l = rng.randint(1, n - width + 1)
        out.append((l, l + width - 1))
    return out


def sized(rng, n):
    """Complexity profile input (ADR 0024): n photos of 500 species, n // 3 wide questions."""
    return render([rng.randint(1, 500) for _ in range(n)], wide(rng, n, max(1, n // 3)))


def generate(seed, size):
    rng = random.Random(seed)
    if isinstance(size, int):
        return sized(rng, size)
    if size == "max":
        if seed % 2 == 1:
            # 5·10⁴ photos of 2000 species and 2·10⁴ questions over 60-100% of the log:
            # collecting each stretch's species is about 8·10⁸ steps. Short ids keep the
            # input small.
            n = 50_000
            return render([rng.randint(1, 2000) for _ in range(n)], wide(rng, n, 20_000))
        # 2·10⁴ photos with ids up to 10⁹ (a few species seen often, many seen once),
        # short and long stretches.
        n = 20_000
        common = [rng.randint(1, 1_000_000_000) for _ in range(30)]
        ids = [rng.choice(common) if rng.random() < 0.7 else rng.randint(1, 1_000_000_000) for _ in range(n)]
        questions = []
        for _ in range(10_000):
            a, b = rng.randint(1, n), rng.randint(1, n)
            questions.append((min(a, b), max(a, b)))
        return render(ids, questions)
    if seed % 5 == 0:
        # Edge shapes: one photo, one species throughout, all different, single-photo
        # stretches, the whole log.
        kind = rng.randrange(3)
        n = rng.randint(1, 8)
        if kind == 0:
            ids = [rng.choice([1, 1_000_000_000])] * n
        elif kind == 1:
            ids = rng.sample(range(1, 1_000_000_001), n)
        else:
            ids = [rng.choice([5, 1_000_000_000]) for _ in range(n)]
        return render(ids, [(1, n)] + [(i, i) for i in range(1, n + 1)][: rng.randint(1, 4)])
    n = rng.randint(1, 12)
    pool = [rng.randint(1, 1_000_000_000) for _ in range(rng.randint(1, 6))]
    ids = [rng.choice(pool) for _ in range(n)]
    questions = []
    for _ in range(rng.randint(1, 8)):
        a, b = rng.randint(1, n), rng.randint(1, n)
        questions.append((min(a, b), max(a, b)))
    return render(ids, questions)


if __name__ == "__main__":
    import sys

    args = sys.argv[1:]
    sys.stdout.write(generate(int(args[0]) if args else 1, args[1] if len(args) > 1 else "small"))
