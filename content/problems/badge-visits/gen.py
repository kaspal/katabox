"""Gate scans and prefix questions as stdin text (a program problem, ADR 0028)."""
import random


def generate(seed, size):
    rng = random.Random(seed)
    if size == "max":
        # 30 000 scans and 30 000 questions with p spread over 1..N (few repeats, so
        # caching answers per p does not help): recounting each prefix from scratch
        # costs about 4.5·10⁸ set insertions. Ids stay short (at most 5 digits) to
        # keep the test lean; the pool size sets how many repeat.
        n = q = 30_000
        pool = rng.sample(range(1, 100_000), rng.randint(10_000, 30_000))
        scans = [rng.choice(pool) for _ in range(n)]
        ps = [rng.randint(1, n) for _ in range(q)]
        ps[0], ps[1], ps[2] = 1, n, n // 2
        rng.shuffle(ps)
    elif seed % 5 == 0:
        # Edge shapes: a single scan, one member all day, or every badge different.
        n = rng.randint(1, 6)
        shape = rng.randrange(3)
        if shape == 0:
            scans = [rng.choice([1, 10**9])] * n
        elif shape == 1:
            scans = rng.sample(range(10**9 - 20, 10**9 + 1), n)
        else:
            scans = [rng.randint(1, 10**9) for _ in range(n)]
        ps = [rng.randint(1, n) for _ in range(rng.randint(1, 5))] + [n, 1]
    else:
        n = rng.randint(1, 8)
        scans = [rng.randint(1, rng.choice([2, 4, 10])) for _ in range(n)]
        ps = [rng.randint(1, n) for _ in range(rng.randint(1, 6))]
    lines = [str(len(scans)), " ".join(map(str, scans)), str(len(ps))]
    lines.extend(map(str, ps))
    return "\n".join(lines) + "\n"


if __name__ == "__main__":
    import sys

    args = sys.argv[1:]
    sys.stdout.write(generate(int(args[0]) if args else 1, args[1] if len(args) > 1 else "small"))
