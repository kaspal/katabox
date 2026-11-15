"""Street maps and ambulance calls as stdin text (a program problem, ADR 0028)."""
import random


def network(rng, linked, count, w_hi, chain):
    """Streets joining junctions 1..linked into one network, plus random extra streets up to `count`."""
    order = list(range(2, linked + 1))
    rng.shuffle(order)
    towns = [1] + order
    roads = []
    for i in range(1, len(towns)):
        # A long chain (deep shortest paths) or a random tree.
        parent = towns[i - 1] if chain else towns[rng.randrange(i)]
        roads.append((parent, towns[i], rng.randint(1, w_hi)))
    while len(roads) < count:
        a, b = rng.sample(towns, 2)
        roads.append((a, b, rng.randint(1, w_hi)))
    rng.shuffle(roads)
    return [(b, a, w) if rng.random() < 0.5 else (a, b, w) for a, b, w in roads]


def sized(rng, n):
    """Complexity profile input (ADR 0024): n junctions, 2n streets (at most 2·10⁴), n calls."""
    roads = network(rng, n, min(2 * n, 20_000), 10_000, chain=False)
    queries = [rng.randint(1, n) for _ in range(n)]
    lines = [f"{n} {len(roads)}"]
    lines += [f"{a} {b} {w}" for a, b, w in roads]
    lines.append(str(len(queries)))
    lines += [str(t) for t in queries]
    return "\n".join(lines) + "\n"


def generate(seed, size):
    rng = random.Random(seed)
    if isinstance(size, int):
        return sized(rng, size)
    if size == "max":
        # 10 000 junctions, 13 000 streets, 10 000 calls (about 290 KB): one search per
        # question is ~10⁴ full searches over the network.
        n = 10_000
        linked = n - rng.randint(5, 40)  # the last few junctions are cut off from the base
        roads = network(rng, linked, 12_990, 10_000, chain=seed % 2 == 0)
        cut = list(range(linked + 1, n + 1))
        for _ in range(10):
            a, b = rng.sample(cut, 2)
            roads.append((a, b, rng.randint(1, 10_000)))
        queries = [rng.randint(1, n) for _ in range(10_000)]
    elif seed % 5 == 0:
        # Edge shapes: a lone depot, no roads, parallel roads, the extreme weights.
        n = rng.randint(1, 6)
        roads = []
        if n >= 2:
            for _ in range(rng.randint(0, 6)):
                a, b = rng.sample(range(1, n + 1), 2)
                roads.append((a, b, rng.choice([1, 10_000])))
                if rng.random() < 0.4:
                    roads.append((b, a, rng.choice([1, 10_000])))
        queries = [rng.randint(1, n) for _ in range(rng.randint(1, 6))]
    else:
        n = rng.randint(2, 9)
        roads = []
        for _ in range(rng.randint(0, 14)):
            a, b = rng.sample(range(1, n + 1), 2)
            roads.append((a, b, rng.randint(1, 20)))
        queries = [rng.randint(1, n) for _ in range(rng.randint(1, 8))]
    lines = [f"{n} {len(roads)}"]
    lines += [f"{a} {b} {w}" for a, b, w in roads]
    lines.append(str(len(queries)))
    lines += [str(t) for t in queries]
    return "\n".join(lines) + "\n"


if __name__ == "__main__":
    import sys

    args = sys.argv[1:]
    sys.stdout.write(generate(int(args[0]) if args else 1, args[1] if len(args) > 1 else "small"))
