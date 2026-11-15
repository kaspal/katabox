"""Jobs with deadlines and fees as stdin text (a program problem, ADR 0028)."""
import random


def render(jobs):
    return "\n".join([str(len(jobs))] + [f"{d} {p}" for d, p in jobs]) + "\n"


def crowded(rng, n):
    """n jobs all due by day n // 2: only half fit, and searching back day by day from
    the deadline for a free day is slow."""
    return [(max(1, n // 2), rng.randint(1, 99)) for _ in range(n)]


def sized(rng, n):
    """Complexity profile input (ADR 0024): n crowded jobs."""
    return render(crowded(rng, n))


def generate(seed, size):
    rng = random.Random(seed)
    if isinstance(size, int):
        return sized(rng, size)
    if size == "max":
        if seed % 2 == 1:
            # 10⁵ jobs due by day 5·10⁴: searching back from the deadline for a free day
            # is about 3.7·10⁹ steps.
            return render(crowded(rng, 100_000))
        # 2·10⁴ jobs, deadlines from 1 to 10⁹ (most beyond the number of jobs), fees up to 10⁹.
        jobs = []
        for _ in range(20_000):
            d = rng.randint(1, 20_000) if rng.random() < 0.8 else rng.randint(1, 1_000_000_000)
            jobs.append((d, rng.randint(1, 1_000_000_000)))
        return render(jobs)
    if seed % 5 == 0:
        # Edge shapes: one job, all due on day 1, equal fees, a deadline of 10⁹, the
        # largest fees.
        kind = rng.randrange(4)
        if kind == 0:
            return render([(rng.choice([1, 1_000_000_000]), rng.choice([1, 1_000_000_000]))])
        if kind == 1:
            return render([(1, rng.randint(1, 9)) for _ in range(rng.randint(2, 6))])
        if kind == 2:
            return render([(rng.randint(1, 3), 5) for _ in range(rng.randint(2, 6))])
        return render([(1_000_000_000, 1_000_000_000)] * 3 + [(1, 1_000_000_000)])
    n = rng.randint(1, 8)
    return render([(rng.randint(1, n + 1), rng.randint(1, 30)) for _ in range(n)])


if __name__ == "__main__":
    import sys

    args = sys.argv[1:]
    sys.stdout.write(generate(int(args[0]) if args else 1, args[1] if len(args) > 1 else "small"))
