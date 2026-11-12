"""Lamp timers."""
import random


def generate(seed, size):
    rng = random.Random(seed)
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

    print(json.dumps(generate(int(sys.argv[1]) if len(sys.argv) > 1 else 1, sys.argv[2] if len(sys.argv) > 2 else "small")))
