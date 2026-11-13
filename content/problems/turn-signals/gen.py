"""Random routes, with many straight ones."""
import random


def route(rng, r):
    while True:
        ax, ay, bx, by = (rng.randint(-r, r) for _ in range(4))
        if (ax, ay) == (bx, by):
            continue
        if rng.random() < 0.3:
            k = rng.choice([-2, -1, 1, 2, 3])
            cx, cy = bx + k * (bx - ax), by + k * (by - ay)
            if abs(cx) > 10**4 or abs(cy) > 10**4:
                continue
        else:
            cx, cy = rng.randint(-r, r), rng.randint(-r, r)
        if (cx, cy) != (bx, by):
            return [ax, ay, bx, by, cx, cy]


def generate(seed, size):
    rng = random.Random(seed)
    if size == "max":
        return {"routes": [route(rng, 10**4) for _ in range(10_000)]}
    return {"routes": [route(rng, 6) for _ in range(rng.randint(1, 6))]}


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
