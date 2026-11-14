"""Random volunteer rosters as stdin text (a program problem, ADR 0028)."""
import random

SYLLABLES = ["ka", "lo", "mi", "ne", "ru", "sa", "to", "vi", "be", "da", "fe", "go", "ha", "ju", "pi", "zo"]
LETTERS = "abcdefghijklmnopqrstuvwxyz"


def make_names(rng, n, longest, odd_share):
    # Distinct names in generation order (never iterate a set: its order is not fixed).
    seen = set()
    names = []
    while len(names) < n:
        if rng.random() < odd_share:
            name = "".join(rng.choice(LETTERS) for _ in range(rng.randint(1, 12)))
        else:
            name = "".join(rng.choice(SYLLABLES) for _ in range(rng.randint(1, longest)))
        if name not in seen:
            seen.add(name)
            names.append(name)
    return names


def generate(seed, size):
    rng = random.Random(seed)
    if size == "max":
        n = 20_000
        if seed % 3 == 0:
            # Everyone at 8 but one at 9: the average sits just above 8.
            hours = [8] * n
            hours[rng.randrange(n)] = 9
        else:
            hours = [rng.randint(0, 24) for _ in range(n)]
        names = make_names(rng, n, 6, 0.0)
    elif seed % 5 == 0:
        # Edge shapes: all equal, only 0 and 24, a single volunteer just above the rest.
        n = rng.randint(1, 5)
        kind = rng.randint(0, 2)
        if kind == 0:
            hours = [rng.choice([0, 24, rng.randint(0, 24)])] * n
        elif kind == 1:
            hours = [rng.choice([0, 24]) for _ in range(n)]
        else:
            base = rng.randint(0, 23)
            hours = [base] * n
            hours[rng.randrange(n)] = base + 1
        names = make_names(rng, n, 2, 0.5)
    else:
        n = rng.randint(1, 6)
        hours = [rng.randint(0, 24) for _ in range(n)]
        names = make_names(rng, n, 3, 0.2)
    lines = [str(n)] + [f"{name} {h}" for name, h in zip(names, hours)]
    return "\n".join(lines) + "\n"


if __name__ == "__main__":
    import sys

    args = sys.argv[1:]
    sys.stdout.write(generate(int(args[0]) if args else 1, args[1] if len(args) > 1 else "small"))
