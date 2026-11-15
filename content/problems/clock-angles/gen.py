"""Times of day as stdin text (a program problem, ADR 0028)."""
import random


def generate(seed, size):
    rng = random.Random(seed)
    if size == "max":
        # Every minute of the day, in order (seed 1) or shuffled (other seeds).
        times = [(h, m) for h in range(24) for m in range(60)]
        if seed != 1:
            rng.shuffle(times)
    elif seed % 5 == 0:
        # Edge shapes: the hours, the half hours, just before and after midnight and noon.
        pool = [(0, 0), (12, 0), (6, 0), (18, 0), (23, 59), (11, 59), (0, 1), (12, 30), (3, 15), (9, 45)]
        times = rng.sample(pool, rng.randint(1, len(pool)))
    else:
        times = [(rng.randrange(24), rng.randrange(60)) for _ in range(rng.randint(1, 8))]
    lines = [str(len(times))] + [f"{h:02d}:{m:02d}" for h, m in times]
    return "\n".join(lines) + "\n"


if __name__ == "__main__":
    import sys

    args = sys.argv[1:]
    sys.stdout.write(generate(int(args[0]) if args else 1, args[1] if len(args) > 1 else "small"))
