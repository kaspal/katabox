"""Random flow-meter logs as stdin text (a program problem, ADR 0028)."""
import random

N_MAX = 60_000


def generate(seed, size):
    rng = random.Random(seed)
    if size == "max":
        if seed % 3 == 0:
            # Long plateaus, with the extreme readings 0 and 10^6.
            readings = []
            while len(readings) < N_MAX:
                value = rng.choice([0, 1_000_000, rng.randint(0, 9999)])
                readings += [value] * rng.randint(1, 3000)
            readings = readings[:N_MAX]
        else:
            # Values below 10^4 keep the text lean (about 5 bytes a reading); 40% repeats.
            readings = []
            value = rng.randint(0, 9999)
            for _ in range(N_MAX):
                if rng.random() < 0.6:
                    value = rng.randint(0, 9999)
                readings.append(value)
    elif seed % 5 == 0:
        # Edge shapes: one reading, all equal, strictly rising or falling, the extremes.
        n = rng.randint(1, 8)
        kind = rng.randint(0, 3)
        if kind == 0:
            readings = [rng.choice([0, 1_000_000, rng.randint(0, 9)])] * n
        elif kind == 1:
            start = rng.randint(0, 5)
            readings = list(range(start, start + n))
        elif kind == 2:
            readings = list(range(n, 0, -1))
        else:
            readings = [rng.choice([0, 1_000_000]) for _ in range(n)]
    else:
        n = rng.randint(1, 10)
        readings = [rng.randint(0, 4) for _ in range(n)]
    return "\n".join(map(str, [len(readings), *readings])) + "\n"


if __name__ == "__main__":
    import sys

    args = sys.argv[1:]
    sys.stdout.write(generate(int(args[0]) if args else 1, args[1] if len(args) > 1 else "small"))
