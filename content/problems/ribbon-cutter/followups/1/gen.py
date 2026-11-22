"""Random inputs for ribbon-cutter--f1 (many orders)."""
import random


def generate(seed, size):
    rng = random.Random(seed)
    if size == "example":
        n, m, q = rng.randint(2, 4), 40, 3
    elif size == "max":
        n, m, q = 50_000, 100_000, 50_000
    else:
        n, m, q = rng.randint(1, 6), rng.choice([5, 30, 1000]), rng.randint(1, 6)
    ribbons = [rng.randint(1, m) for _ in range(n)]
    total = sum(ribbons)
    orders = []
    for _ in range(q):
        if rng.random() < 0.15:
            orders.append(rng.randint(total + 1, max(total + 1, 10**10)))
        else:
            orders.append(rng.randint(1, max(1, total)))
    return {"ribbons": ribbons, "orders": orders}


if __name__ == "__main__":
    import json
    import sys

    args = sys.argv[1:]
    print(json.dumps(generate(int(args[0]) if args else 1, args[1] if len(args) > 1 else "small")))
