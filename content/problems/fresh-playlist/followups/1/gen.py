"""Random play logs and requests for fresh-playlist--f1."""
import random


def generate(seed, size):
    rng = random.Random(seed)
    if size == "max":
        n, q, songs = 100_000, 15_000, rng.choice([300, 20_000])
    elif size == "example":
        n, q, songs = 7, 3, 4
    else:
        n, q, songs = rng.randint(1, 10), rng.randint(1, 6), rng.choice([2, 4, 30])
    plays = [rng.randint(1, songs) for _ in range(n)]
    requests = []
    for _ in range(q):
        a, b = rng.randrange(n), rng.randrange(n)
        if size == "max" and rng.random() < 0.7:
            a, b = rng.randrange(n // 10), rng.randrange(n - n // 10, n)
        requests.append([min(a, b), max(a, b)])
    return {"plays": plays, "requests": requests}


if __name__ == "__main__":
    import json
    import sys

    args = sys.argv[1:]
    print(json.dumps(generate(int(args[0]) if args else 1, args[1] if len(args) > 1 else "small")))
