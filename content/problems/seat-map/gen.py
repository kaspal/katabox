"""Halls and bookings for seat-map."""
import random


def generate(seed, size):
    rng = random.Random(seed)
    if size == "max":
        rows, cols, b = 300, 300, 10**4
    elif size == "example":
        rows, cols, b = rng.randint(2, 3), rng.randint(3, 4), rng.randint(1, 3)
    else:
        rows, cols, b = rng.randint(1, 5), rng.randint(1, 6), rng.randint(0, 6)
    bookings = [[rng.randrange(rows), rng.randrange(cols)] for _ in range(b)]
    return {"rows": rows, "cols": cols, "bookings": bookings}


if __name__ == "__main__":
    import json
    import sys

    args = sys.argv[1:]
    print(json.dumps(generate(int(args[0]) if args else 1, args[1] if len(args) > 1 else "small")))
