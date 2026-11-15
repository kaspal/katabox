"""Exam scores and rank questions as stdin text (a program problem, ADR 0028)."""
import random


def sized(rng, n):
    """Complexity profile input (ADR 0024): n scores and n rank questions, like "max"."""
    pool = rng.sample(range(1_000_000), max(1, n // 2))
    scores = [rng.choice(pool) for _ in range(n)]
    asked = [rng.choice(scores) if rng.random() < 0.85 else rng.randrange(1_000_000) for _ in range(n)]
    lines = [str(n), " ".join(map(str, scores)), str(n), " ".join(map(str, asked))]
    return "\n".join(lines) + "\n"


def generate(seed, size):
    rng = random.Random(seed)
    if isinstance(size, int):
        return sized(rng, size)
    if size == "max":
        # 20 000 scores and 20 000 questions, most of them scores someone got, so
        # scanning every score per question costs about 4·10⁸ steps. Scores stay below
        # 10⁶ to keep the test lean; the pool size sets how many repeat.
        n = q = 20_000
        pool = rng.sample(range(1_000_000), rng.randint(8_000, 20_000))
        scores = [rng.choice(pool) for _ in range(n)]
        asked = [rng.choice(scores) if rng.random() < 0.85 else rng.randrange(1_000_000) for _ in range(q)]
        asked[0], asked[1], asked[2] = max(scores), min(scores), 1_000_000_000
        rng.shuffle(asked)
    elif seed % 5 == 0:
        # Edge shapes: everyone tied, the extreme scores, a single student.
        n = rng.randint(1, 6)
        if rng.random() < 0.5:
            scores = [rng.choice([0, 10**9])] * n
        else:
            scores = [rng.choice([0, 1, 10**9 - 1, 10**9]) for _ in range(n)]
        asked = [rng.choice([0, 1, 2, 10**9 - 1, 10**9]) for _ in range(rng.randint(1, 6))]
    else:
        n = rng.randint(1, 8)
        top = rng.choice([3, 10, 100])
        scores = [rng.randint(0, top) for _ in range(n)]
        asked = [rng.choice(scores) if rng.random() < 0.6 else rng.randint(0, top + 1) for _ in range(rng.randint(1, 6))]
    lines = [str(len(scores)), " ".join(map(str, scores)), str(len(asked)), " ".join(map(str, asked))]
    return "\n".join(lines) + "\n"


if __name__ == "__main__":
    import sys

    args = sys.argv[1:]
    sys.stdout.write(generate(int(args[0]) if args else 1, args[1] if len(args) > 1 else "small"))
