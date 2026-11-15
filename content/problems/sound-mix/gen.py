"""Sound-source logs as stdin text (a program problem, ADR 0028)."""
import random


def render(k, ids):
    return f"{len(ids)} {k}\n" + " ".join(map(str, ids)) + "\n"


def sized(rng, n):
    """Complexity profile input (ADR 0024): n seconds from 30 sources, K = 20."""
    return render(20, [rng.randint(1, 30) for _ in range(n)])


def generate(seed, size):
    rng = random.Random(seed)
    if isinstance(size, int):
        return sized(rng, size)
    if size == "max":
        if seed % 2 == 1:
            # 10⁵ seconds from 40 sources, K = 40: almost every stretch that reaches all 40
            # keeps going to the end, so extending from each start is about 5·10⁹ steps.
            # The ids are short so the input stays small.
            ids = [rng.randint(1, 40) for _ in range(100_000)]
            k = 40
        else:
            # 5·10⁴ seconds in long runs of one source each (K = 3), ids up to 10⁹.
            pool = [rng.randint(1, 1_000_000_000) for _ in range(200)]
            ids = []
            while len(ids) < 50_000:
                ids += [rng.choice(pool)] * rng.randint(1, 300)
            ids = ids[:50_000]
            k = 3
    elif seed % 5 == 0:
        # Edge shapes: one second, one source, K = n with all different, K above the
        # number of sources, the largest ids.
        kind = rng.randrange(4)
        if kind == 0:
            ids, k = [rng.randint(1, 1_000_000_000)], 1
        elif kind == 1:
            ids = [rng.choice([1, 1_000_000_000])] * rng.randint(1, 8)
            k = rng.randint(1, len(ids))
        elif kind == 2:
            ids = rng.sample(range(1, 1_000_000_001), rng.randint(1, 8))
            k = len(ids)
        else:
            ids = [rng.choice([7, 1_000_000_000]) for _ in range(rng.randint(3, 8))]
            k = 3
    else:
        n = rng.randint(1, 12)
        pool = [rng.randint(1, 1_000_000_000) for _ in range(rng.randint(1, 5))]
        ids = [rng.choice(pool) for _ in range(n)]
        k = rng.randint(1, n)
    return render(k, ids)


if __name__ == "__main__":
    import sys

    args = sys.argv[1:]
    sys.stdout.write(generate(int(args[0]) if args else 1, args[1] if len(args) > 1 else "small"))
