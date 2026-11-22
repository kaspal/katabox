"""Friendships for club-count: few people, many friendships, so clubs chain up."""
import random


def generate(seed, size):
    rng = random.Random(seed)
    if size == "max":
        n, m = 10**4, 2 * 10**4
    elif size == "example":
        n, m = rng.randint(4, 6), rng.randint(3, 5)
    else:
        n, m = rng.randint(2, 9), rng.randint(1, 12)
    friendships = [rng.sample(range(n), 2) for _ in range(m)]
    return {"n": n, "friendships": friendships}


if __name__ == "__main__":
    import json
    import sys

    args = sys.argv[1:]
    print(json.dumps(generate(int(args[0]) if args else 1, args[1] if len(args) > 1 else "small")))
