"""Small character pictures and zoom factors as stdin text (a program problem, ADR 0028)."""
import random

INK = ".#*o+"


def picture(rng, r, c, ink):
    return ["".join(rng.choice(ink) for _ in range(c)) for _ in range(r)]


def generate(seed, size):
    rng = random.Random(seed)
    if size == "max":
        # The largest picture and zoom: 320 lines of 320 characters out.
        r, c, k = 40, 40, 8
        rows = picture(rng, r, c, INK)
    elif seed % 5 == 0:
        # Edge shapes: one pixel, one row, one column, zoom 1, a blank picture.
        kind = rng.randrange(4)
        r, c = [(1, 1), (1, rng.randint(2, 9)), (rng.randint(2, 9), 1), (rng.randint(1, 5), rng.randint(1, 5))][kind]
        k = rng.choice([1, 8]) if kind < 3 else 1
        rows = picture(rng, r, c, "." if rng.random() < 0.5 else INK)
    else:
        r, c, k = rng.randint(1, 6), rng.randint(1, 6), rng.randint(1, 4)
        rows = picture(rng, r, c, INK)
    return "\n".join([f"{r} {c} {k}", *rows]) + "\n"


if __name__ == "__main__":
    import sys

    args = sys.argv[1:]
    sys.stdout.write(generate(int(args[0]) if args else 1, args[1] if len(args) > 1 else "small"))
