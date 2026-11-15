"""Park trail maps (trees) as stdin text (a program problem, ADR 0028)."""
import random


def tree(rng, n, shape):
    """Parent links for junctions 2..n, then randomly relabelled and shuffled edges."""
    edges = []
    if shape == "random":
        edges = [(rng.randint(1, i - 1), i) for i in range(2, n + 1)]
    elif shape == "chain":
        # Almost a path: every junction hangs off one of the last three.
        edges = [(max(1, i - 1 - rng.randint(0, 2)), i) for i in range(2, n + 1)]
    elif shape == "caterpillar":
        spine = max(1, n // 2)
        edges = [(i - 1, i) for i in range(2, spine + 1)]
        edges += [(rng.randint(1, spine), i) for i in range(spine + 1, n + 1)]
    elif shape == "star":
        edges = [(1, i) for i in range(2, n + 1)]
    else:  # "local": each junction joins one of the 50 before it
        edges = [(rng.randint(max(1, i - 50), i - 1), i) for i in range(2, n + 1)]
    label = list(range(1, n + 1))
    rng.shuffle(label)
    out = []
    for a, b in edges:
        a, b = label[a - 1], label[b - 1]
        out.append((a, b) if rng.random() < 0.5 else (b, a))
    rng.shuffle(out)
    return out


def sized(rng, n):
    """Complexity profile input (ADR 0024): one park of n junctions, a random tree."""
    lines = ["1", str(n)]
    lines.extend(f"{a} {b}" for a, b in tree(rng, n, "random"))
    return "\n".join(lines) + "\n"


def generate(seed, size):
    rng = random.Random(seed)
    if isinstance(size, int):
        return sized(rng, size)
    if size == "max":
        # One park with 20 000 junctions: a BFS from every junction costs 4·10⁸ steps.
        # Odd seeds: T = 1000, the big park plus 999 parks of 1-2 junctions (the big
        # one keeps about 18 500, still about 3.4·10⁸ steps for that approach).
        shape = ["random", "chain", "caterpillar", "local"][seed % 4]
        tiny = [rng.randint(1, 2) for _ in range(999)] if seed % 2 else []
        big = 20_000 - sum(tiny)
        parks = [(n, tree(rng, n, "chain")) for n in tiny]
        parks.insert(rng.randint(0, len(parks)), (big, tree(rng, big, shape)))
    elif seed % 5 == 0:
        # Edge shapes: a single junction, two junctions, a star, a path.
        parks = []
        for _ in range(rng.randint(1, 3)):
            n = rng.choice([1, 2, rng.randint(3, 8)])
            parks.append((n, tree(rng, n, rng.choice(["star", "chain", "caterpillar"]))))
    else:
        parks = []
        for _ in range(rng.randint(1, 3)):
            n = rng.randint(1, 9)
            parks.append((n, tree(rng, n, rng.choice(["random", "chain", "caterpillar", "star", "local"]))))
    lines = [str(len(parks))]
    for n, edges in parks:
        lines.append(str(n))
        lines.extend(f"{a} {b}" for a, b in edges)
    return "\n".join(lines) + "\n"


if __name__ == "__main__":
    import sys

    args = sys.argv[1:]
    sys.stdout.write(generate(int(args[0]) if args else 1, args[1] if len(args) > 1 else "small"))
