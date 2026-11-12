"""Lamp timers."""
import random


def generate(seed, size):
    rng = random.Random(seed)
    if isinstance(size, int):
        # Profile (ADR 0024): n short lamps (small times keep the data lean).
        lamps = []
        for _ in range(size):
            start = rng.randint(0, 10**7)
            lamps.append([start, start + rng.randint(1, 3_000)])
        return {"lamps": lamps}
    if size == "max":
        # 3·10⁴ short lamps over 10⁸ seconds: checking each of the 6·10⁴ pieces between
        # cuts against the lamps is about 1.8·10⁹ steps (most pieces are dark).
        lamps = []
        for _ in range(30_000):
            start = rng.randint(0, 100_000_000)
            lamps.append([start, start + rng.randint(1, 3_000)])
        return {"lamps": lamps}
    hi = rng.choice([10, 30, 1000])
    lamps = []
    for _ in range(rng.randint(1, 6)):
        start = rng.randint(0, hi - 1)
        lamps.append([start, rng.randint(start + 1, min(hi, start + max(1, hi // 3)))])
    return {"lamps": lamps}


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
