"""Random meshes and ladders for fewest-hops--f1."""
import random


def ladder(levels, width):
    # 0, then `levels` levels of `width` stations, each joined to every station of the
    # next level, then n - 1: width ** levels fewest routes. Labels in order (compact).
    n = 2 + levels * width
    links = []
    prev = [0]
    nxt = 1
    for _ in range(levels):
        level = list(range(nxt, nxt + width))
        nxt += width
        links += [[a, b] for a in prev for b in level]
        prev = level
    links += [[a, n - 1] for a in prev]
    return n, links


def generate(seed, size):
    rng = random.Random(seed)
    if size == "max" and seed % 2 == 0:
        n, links = ladder(rng.choice([24_000, 33_000]), 2)
        return {"n": n, "links": links}
    if size == "max":
        n, m = 20_000, 40_000
    elif size == "example":
        n, m = rng.randint(4, 7), rng.randint(3, 9)
    else:
        n = rng.randint(1, 9)
        m = rng.randint(0, min(14, n * (n - 1) // 2))
    if n >= 2 and rng.random() < 0.3 and size != "max":
        n, links = ladder(rng.randint(1, 3), rng.randint(1, 2))
        return {"n": n, "links": links}
    pairs = set()
    while len(pairs) < m:
        a, b = rng.sample(range(n), 2)
        pairs.add((min(a, b), max(a, b)))
    links = [[a, b] if rng.random() < 0.5 else [b, a] for a, b in sorted(pairs)]
    rng.shuffle(links)
    return {"n": n, "links": links}


if __name__ == "__main__":
    import json
    import sys

    args = sys.argv[1:]
    print(json.dumps(generate(int(args[0]) if args else 1, args[1] if len(args) > 1 else "small")))
