"""User settings and questions."""
import random


def generate(seed, size):
    rng = random.Random(seed)
    if size == "max":
        # 10⁵ users and 10⁵ questions: testing every pair is 10¹⁰ checks. 8-bit masks
        # keep the data lean; one value uses the top bit, so the table has all 2¹⁶
        # entries.
        n = q = 100_000
        users = [rng.getrandbits(8) for _ in range(n)]
        users[0] = 65_535
        queries = [rng.getrandbits(8) | rng.getrandbits(8) for _ in range(q)]
        return {"users": users, "queries": queries}
    bits = rng.choice([2, 4, 16])
    users = [rng.getrandbits(bits) for _ in range(rng.randint(1, 8))]
    queries = [rng.getrandbits(bits) & rng.getrandbits(bits) for _ in range(rng.randint(1, 6))]
    return {"users": users, "queries": queries}


if __name__ == "__main__":
    import json
    import sys

    print(json.dumps(generate(int(sys.argv[1]) if len(sys.argv) > 1 else 1, sys.argv[2] if len(sys.argv) > 2 else "small")))
