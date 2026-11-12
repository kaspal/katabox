"""Box labels."""
import random


def label(rng, length_hi):
    parts = []
    letters = rng.random() < 0.5
    for _ in range(rng.randint(1, 3)):
        if letters:
            parts.append("".join(rng.choice("abx") for _ in range(rng.randint(1, 3))))
        else:
            parts.append(str(rng.randint(0, 120)).zfill(rng.choice([0, 0, 2, 3])))
        letters = not letters
    return "".join(parts)[:length_hi]


def generate(seed, size):
    rng = random.Random(seed)
    if size == "max":
        n = 10_000
        labels = []
        for _ in range(n):
            s = rng.choice(["box", "shelf", "bin"]) + str(rng.randint(0, 10**rng.randint(1, 15))).zfill(rng.choice([0, 3]))
            if rng.random() < 0.3:
                s += rng.choice("abc")
            labels.append(s[:20])
        return {"labels": labels}
    return {"labels": [label(rng, 20) or "a" for _ in range(rng.randint(1, 8))]}


if __name__ == "__main__":
    import json
    import sys

    print(json.dumps(generate(int(sys.argv[1]) if len(sys.argv) > 1 else 1, sys.argv[2] if len(sys.argv) > 2 else "small")))
