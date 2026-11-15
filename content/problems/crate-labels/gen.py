"""Warehouse crate labels as stdin text (a program problem, ADR 0028)."""
import random
import string


def word(rng, low=1, high=10):
    return "".join(rng.choice(string.ascii_lowercase) for _ in range(rng.randint(low, high)))


def render(labels):
    lines = [str(len(labels))]
    lines += [";".join(f"{name}:{qty}" for name, qty in label) for label in labels]
    return "\n".join(lines) + "\n"


def generate(seed, size):
    rng = random.Random(seed)
    if size == "max":
        # 1000 labels of 20 entries from 300 products, quantities up to 1000.
        names = list({word(rng, 3, 10) for _ in range(300)})
        return render([[(rng.choice(names), rng.randint(1, 1000)) for _ in range(20)] for _ in range(1000)])
    if seed % 5 == 0:
        # Edge shapes: one entry, a name that begins another (fig, figs), one product many
        # times, one-letter and ten-letter names, the largest quantity.
        kind = rng.randrange(3)
        if kind == 0:
            return render([[(word(rng, 10, 10), 1000)]])
        if kind == 1:
            return render([[("fig", 2), ("figs", 3), ("f", 1)], [("figs", 1), ("fig", 1000)]])
        return render([[("a", rng.randint(1, 9))] * 20 for _ in range(rng.randint(1, 3))])
    names = [word(rng, 1, 6) for _ in range(rng.randint(1, 5))]
    return render([[(rng.choice(names), rng.randint(1, 50)) for _ in range(rng.randint(1, 4))] for _ in range(rng.randint(1, 4))])


if __name__ == "__main__":
    import sys

    args = sys.argv[1:]
    sys.stdout.write(generate(int(args[0]) if args else 1, args[1] if len(args) > 1 else "small"))
