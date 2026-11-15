"""Print jobs (arrival, priority, pages) as stdin text (a program problem, ADR 0028)."""
import random


def render(jobs):
    return "\n".join([str(len(jobs))] + [f"{a} {p} {g}" for a, p, g in jobs]) + "\n"


def busy_office(rng, n):
    """Three jobs arrive every second, but a job takes 5 seconds on average: the queue
    grows to most of the jobs."""
    return [(i // 3, rng.randint(1, 100), rng.randint(1, 9)) for i in range(n)]


def sized(rng, n):
    """Complexity profile input (ADR 0024): n jobs from a busy office."""
    return render(busy_office(rng, n))


def generate(seed, size):
    rng = random.Random(seed)
    if isinstance(size, int):
        return sized(rng, size)
    if size == "max":
        if seed % 2 == 1:
            # 5·10⁴ jobs from a busy office: scanning the whole queue for every pick is
            # about 10⁹ comparisons.
            return render(busy_office(rng, 50_000))
        # 10⁴ jobs in shuffled order, far-apart arrivals up to 10⁹ (the printer idles),
        # long jobs, priorities up to 10⁹.
        jobs = [(rng.randint(0, 1_000_000_000), rng.randint(1, 1_000_000_000), rng.randint(1, 10_000)) for _ in range(10_000)]
        return render(jobs)
    if seed % 5 == 0:
        # Edge shapes: one job, equal jobs (arrival, then number decide), a gap where the
        # printer waits, a late urgent job.
        kind = rng.randrange(4)
        if kind == 0:
            return render([(rng.randint(0, 10), rng.randint(1, 5), rng.randint(1, 5))])
        if kind == 1:
            return render([(3, 7, 2)] * rng.randint(2, 5))
        if kind == 2:
            return render([(100, 1, 5), (0, 1, 5), (1_000_000_000, 2, 10_000)])
        return render([(0, 1, 10), (0, 1, 10), (1, 9, 1), (25, 5, 1)])
    jobs = [(rng.randint(0, 12), rng.randint(1, 4), rng.randint(1, 4)) for _ in range(rng.randint(1, 8))]
    return render(jobs)


if __name__ == "__main__":
    import sys

    args = sys.argv[1:]
    sys.stdout.write(generate(int(args[0]) if args else 1, args[1] if len(args) > 1 else "small"))
