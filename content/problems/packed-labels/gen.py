"""Packed labels and position checks as stdin text (a program problem, ADR 0028)."""
import random
import string


def pack(runs):
    return "".join(f"{c}{ch}" for c, ch in runs)


def sized(rng, n):
    """Complexity profile input (ADR 0024): one label of n short runs and n // 3 checks
    near its end, like "max"."""
    runs = [(rng.randint(1, 9), rng.choice("abcdef")) for _ in range(n)]
    total = sum(c for c, _ in runs)
    low = max(1, total - total // 10)
    q = max(1, n // 3)
    lines = ["1", pack(runs), str(q)]
    lines.extend(f"1 {rng.randint(low, total)}" for _ in range(q))
    return "\n".join(lines) + "\n"


def generate(seed, size):
    rng = random.Random(seed)
    if isinstance(size, int):
        return sized(rng, size)
    if size == "max":
        # One long label of short runs plus short labels with huge counts, 60 000
        # runs in all: 3 labels of 20 runs, or (odd seeds) 999 single-run labels so
        # L = 1000. 18 000 checks, 95% near the end of the long label, so walking the
        # runs for each check costs about 10⁹ steps.
        letters = string.ascii_lowercase[: rng.randint(2, 26)]
        two_digit = rng.uniform(0.05, 0.15)
        shorts, short_runs = (999, 1) if seed % 2 else (3, 20)
        long_runs = [
            (rng.randint(10, 99) if rng.random() < two_digit else rng.randint(1, 9), rng.choice(letters))
            for _ in range(60_000 - shorts * short_runs)
        ]
        labels = [long_runs]
        for _ in range(shorts):
            labels.append([(rng.randint(1, 10**9), rng.choice(letters)) for _ in range(short_runs)])
        q = 18_000
    elif seed % 5 == 0:
        # Edge shapes: huge counts, single runs, repeated letters in a row.
        labels = []
        for _ in range(rng.randint(1, 3)):
            ch = rng.choice("xyz")
            labels.append(
                [(rng.choice([1, 10**9, rng.randint(1, 10**9)]), ch if rng.random() < 0.5 else rng.choice("xyz"))
                 for _ in range(rng.randint(1, 4))]
            )
        q = rng.randint(1, 6)
    else:
        letters = rng.choice(["ab", "abc", string.ascii_lowercase])
        labels = [
            [(rng.randint(1, rng.choice([1, 3, 12])), rng.choice(letters)) for _ in range(rng.randint(1, 5))]
            for _ in range(rng.randint(1, 3))
        ]
        q = rng.randint(1, 6)

    totals = []
    for runs in labels:
        ends, t = [], 0
        for c, _ in runs:
            t += c
            ends.append(t)
        totals.append(ends)
    checks = []
    if size == "max":
        ends = totals[0]
        low = ends[int(len(ends) * 0.9)]
        for _ in range(q):
            if rng.random() < 0.95:
                checks.append((1, rng.randint(low, ends[-1])))
            else:
                i = rng.randint(2, len(labels))
                checks.append((i, rng.randint(1, totals[i - 1][-1])))
        for i in [1] + rng.sample(range(2, len(labels) + 1), min(20, len(labels) - 1)):
            checks[rng.randrange(q)] = (i, 1)
            checks[rng.randrange(q)] = (i, totals[i - 1][-1])
    else:
        for _ in range(q):
            i = rng.randint(1, len(labels))
            ends = totals[i - 1]
            pick = rng.random()
            if pick < 0.3:
                # A run boundary: the last position of a run, or the first of the next.
                r = rng.randrange(len(ends))
                k = ends[r] if pick < 0.15 or r == len(ends) - 1 else ends[r] + 1
            else:
                k = rng.randint(1, ends[-1])
            checks.append((i, k))

    lines = [str(len(labels))]
    lines.extend(pack(runs) for runs in labels)
    lines.append(str(len(checks)))
    lines.extend(f"{i} {k}" for i, k in checks)
    return "\n".join(lines) + "\n"


if __name__ == "__main__":
    import sys

    args = sys.argv[1:]
    sys.stdout.write(generate(int(args[0]) if args else 1, args[1] if len(args) > 1 else "small"))
