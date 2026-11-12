"""Floor sizes."""
import random


def generate(seed, size):
    rng = random.Random(seed)
    if size == "max":
        # Two large consecutive numbers share no factor: trying sizes downwards takes
        # about 10¹² steps, Euclid a few dozen.
        a = rng.randint(9 * 10**11, 10**12 - 1)
        return {"width": a + 1, "height": a} if seed % 2 else {"width": a, "height": a + 1}
    g = rng.randint(1, 30)
    return {"width": g * rng.randint(1, 40), "height": g * rng.randint(1, 40)}


if __name__ == "__main__":
    import json
    import sys

    print(json.dumps(generate(int(sys.argv[1]) if len(sys.argv) > 1 else 1, sys.argv[2] if len(sys.argv) > 2 else "small")))
