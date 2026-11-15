"""Gig offers (start, end, pay) as stdin text (a program problem, ADR 0028)."""
import random


def render(gigs):
    return "\n".join([str(len(gigs))] + [f"{s} {e} {p}" for s, e, p in gigs]) + "\n"


def season(rng, n):
    """n overlapping offers through a season, listed by start time: lengths in steps of
    100, small pays (short, repetitive text keeps the input small)."""
    gigs = []
    for i in range(n):
        s = i * 15 + rng.randint(0, 14)
        gigs.append((s, s + 100 * rng.randint(1, 30), rng.randint(1, 20)))
    return gigs


def sized(rng, n):
    """Complexity profile input (ADR 0024): n offers through a season."""
    return render(season(rng, n))


def generate(seed, size):
    rng = random.Random(seed)
    if isinstance(size, int):
        return sized(rng, size)
    if size == "max":
        if seed % 2 == 1:
            # 5·10⁴ offers: trying every earlier offer for each one is about 1.25·10⁹ steps.
            return render(season(rng, 50_000))
        # 5000 offers in random order, times and pays up to 10⁹, many equal pays.
        gigs = []
        for _ in range(5_000):
            s = rng.randint(0, 999_000_000)
            gigs.append((s, s + rng.randint(1, 1_000_000), rng.choice([1_000_000_000, rng.randint(1, 1_000_000_000)])))
        return render(gigs)
    if seed % 5 == 0:
        # Edge shapes: one offer, back-to-back offers, the same total with more or fewer
        # gigs, offers that all overlap, the largest values.
        kind = rng.randrange(4)
        if kind == 0:
            return render([(0, 1_000_000_000, 1_000_000_000)])
        if kind == 1:
            return render([(0, 5, 3), (5, 10, 3), (0, 10, 6), (10, 11, 1)])
        if kind == 2:
            return render([(0, 10, rng.randint(1, 9)) for _ in range(rng.randint(2, 5))])
        return render([(i, i + 1, 1_000_000_000) for i in range(rng.randint(2, 6))])
    gigs = []
    for _ in range(rng.randint(1, 8)):
        s = rng.randint(0, 12)
        gigs.append((s, s + rng.randint(1, 6), rng.randint(1, 6)))
    return render(gigs)


if __name__ == "__main__":
    import sys

    args = sys.argv[1:]
    sys.stdout.write(generate(int(args[0]) if args else 1, args[1] if len(args) > 1 else "small"))
