"""Random roads; the big ones need many stops."""
import random


def road(rng, n, gap, top):
    stations, pos = [], 0
    for _ in range(n):
        pos += rng.randint(1, gap)
        stations.append([pos, rng.randint(1, top)])
    return stations


def generate(seed, size):
    rng = random.Random(seed)
    if size == "max":
        stations = road(rng, 100_000, 60, 99)
        target = stations[-1][0] + rng.randint(1, 60)
        return {"target": target, "fuel": rng.randint(50, 150), "stations": stations}
    stations = road(rng, rng.randint(0, 6), 8, 10)
    target = (stations[-1][0] if stations else 0) + rng.randint(1, 8)
    return {"target": target, "fuel": rng.randint(1, 12), "stations": stations}


if __name__ == "__main__":
    import json
    import sys

    args = sys.argv[1:]
    if "--n" in args:
        # Profile size (ADR 0024): python gen.py --n N [--seed S]
        seed = int(args[args.index("--seed") + 1]) if "--seed" in args else 1
        print(json.dumps(generate(seed, int(args[args.index("--n") + 1]))))
    else:
        print(json.dumps(generate(int(args[0]) if args else 1, args[1] if len(args) > 1 else "small")))
