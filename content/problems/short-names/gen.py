"""Member names."""
import random


def generate(seed, size):
    rng = random.Random(seed)
    if size == "max":
        # 2·10⁴ names sharing long beginnings: comparing every pair of names reads
        # about 4·10⁸ pairs, several letters deep.
        n, head, tail = 20_000, (8, "ab"), (4, 12)
    else:
        n, head, tail = rng.randint(1, 8), (rng.randint(0, 2), "ab"), (1, 3)
    names = set()
    while len(names) < n:
        start = "".join(rng.choice(head[1]) for _ in range(head[0]))
        word = start + "".join(rng.choice("abc") for _ in range(rng.randint(*tail)))
        names.add(word)
        if size != "max" and rng.random() < 0.3 and len(word) > 1:
            names.add(word[: rng.randint(1, len(word) - 1)])
    names = sorted(names)[:n] if size == "max" else sorted(names)
    rng.shuffle(names)
    return {"names": names}


if __name__ == "__main__":
    import json
    import sys

    print(json.dumps(generate(int(sys.argv[1]) if len(sys.argv) > 1 else 1, sys.argv[2] if len(sys.argv) > 2 else "small")))
