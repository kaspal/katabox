"""Seasons of results."""
import random


def generate(seed, size):
    rng = random.Random(seed)
    if size == "max":
        # 10⁵ matches: extending every run is 5·10⁹ steps.
        return {"results": "".join(rng.choice("WLD") for _ in range(100_000))}
    letters = rng.choice(["WL", "WLD", "D", "WWLD"])
    return {"results": "".join(rng.choice(letters) for _ in range(rng.randint(1, 12)))}


if __name__ == "__main__":
    import json
    import sys

    print(json.dumps(generate(int(sys.argv[1]) if len(sys.argv) > 1 else 1, sys.argv[2] if len(sys.argv) > 2 else "small")))
