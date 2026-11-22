"""Random race results for race-results: many equal times."""
import random

NAMES = ["ada", "ben", "cai", "dia", "eli", "fay", "gus", "hal", "ivy", "joe", "kim", "lea"]


def generate(seed, size):
    rng = random.Random(seed)
    if size == "max":
        n = 10**4
        names = [f"r{i:05d}" for i in rng.sample(range(10**5), n)]
    else:
        n = {"example": 4}.get(size, rng.randint(1, 8))
        names = rng.sample(NAMES, n)
    k = rng.choice([2, 3, 600])
    times = [rng.randint(100, 100 + k) for _ in range(n)]
    return {"names": names, "times": times}


if __name__ == "__main__":
    import json
    import sys

    args = sys.argv[1:]
    print(json.dumps(generate(int(args[0]) if args else 1, args[1] if len(args) > 1 else "small")))
