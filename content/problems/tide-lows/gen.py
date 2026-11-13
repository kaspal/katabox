"""Tide logs and stretch questions as stdin text (a program problem, ADR 0028)."""
import random


def generate(seed, size):
    rng = random.Random(seed)
    if size == "max":
        # 10⁵ readings and 5·10⁴ questions, each covering 80-100% of the log: scanning
        # every stretch is ~4.5·10⁹ steps. Calm water is one digit (about 500 KB of text);
        # the lows that decide the answers are placed so that long stretches have different
        # minima (the global minimum is rarely inside one).
        n, q = 100_000, 50_000
        levels = [rng.randint(0, 9) for _ in range(n)]
        shape = seed % 3
        if shape == 0:
            # (a) A falling or rising tide: a low every 100 readings, each deeper than the
            # one before (falling) or after (rising). The answer follows r or l.
            spots = list(range(rng.randrange(100), n, 100))
            falling = rng.random() < 0.5
            for j, p in enumerate(spots):
                step = j if falling else len(spots) - 1 - j
                levels[p] = -(1_000 + 900 * step + rng.randrange(900))
        elif shape == 1:
            # (b) Storm lows near both ends, deeper towards the ends of the log: the answer
            # is the deeper of the first low after l and the last low before r.
            for p in rng.sample(range(5_000), 150):
                levels[p] = -(1_000 + 180 * (5_000 - p) + rng.randrange(180))
            for p in rng.sample(range(95_000, n), 150):
                levels[p] = -(1_090 + 180 * (p - 95_000) + rng.randrange(180))
        else:
            # (c) Lows of random depth every 100 readings.
            for p in range(rng.randrange(100), n, 100):
                levels[p] = -rng.randint(1_000, 1_000_000)
        questions = []
        for _ in range(q):
            if shape == 2:
                # Random stretches of 80-100% of the log.
                length = rng.randint(80_000, n)
                l = rng.randint(1, n - length + 1)
                questions.append((l, l + length - 1))
            else:
                questions.append((rng.randint(1, 5_000), rng.randint(95_001, n)))
    elif seed % 5 == 0:
        # Edge shapes: one reading, equal levels, the extreme levels, whole-log stretches.
        n = rng.randint(1, 8)
        kind = rng.randrange(3)
        if kind == 0:
            levels = [rng.choice([-1_000_000, 1_000_000])] * n
        elif kind == 1:
            levels = [rng.choice([-1_000_000, 0, 1_000_000]) for _ in range(n)]
        else:
            levels = [rng.randint(-2, 2) for _ in range(n)]
        questions = [(1, n)] + [(i, i) for i in range(1, n + 1)][: rng.randint(0, 3)]
    else:
        n = rng.randint(1, 12)
        levels = [rng.randint(-20, 20) for _ in range(n)]
        questions = []
        for _ in range(rng.randint(1, 10)):
            a, b = rng.randint(1, n), rng.randint(1, n)
            questions.append((min(a, b), max(a, b)))
    lines = [str(n), " ".join(map(str, levels)), str(len(questions))]
    lines += [f"{l} {r}" for l, r in questions]
    return "\n".join(lines) + "\n"


if __name__ == "__main__":
    import sys

    args = sys.argv[1:]
    sys.stdout.write(generate(int(args[0]) if args else 1, args[1] if len(args) > 1 else "small"))
