"""Running-club lap sheets as stdin text (a program problem, ADR 0028)."""
import random
import string


def name(rng, length=None):
    return "".join(rng.choice(string.ascii_lowercase) for _ in range(length or rng.randint(1, 8)))


def lap(rng, low, high):
    s = rng.randint(low, high)
    return f"{s // 60}:{s % 60:02d}"


def generate(seed, size):
    rng = random.Random(seed)
    if size == "max":
        # 100 runners with 50 laps each, from sprints to 59:59 walks.
        runners = [(name(rng, rng.randint(1, 12)), [lap(rng, 1, 3599) for _ in range(50)]) for _ in range(100)]
    elif seed % 5 == 0:
        # Edge shapes: one lap, the shortest and longest laps, equal laps, long names.
        runners = []
        for _ in range(rng.randint(1, 4)):
            kind = rng.randrange(3)
            if kind == 0:
                laps = [rng.choice(["0:01", "59:59"])]
            elif kind == 1:
                laps = [rng.choice(["0:59", "1:00", "10:00"])] * rng.randint(2, 5)
            else:
                laps = [rng.choice(["0:01", "0:09", "0:10", "59:59"]) for _ in range(rng.randint(2, 6))]
            runners.append((name(rng, rng.choice([1, 12])), laps))
    else:
        runners = [(name(rng), [lap(rng, 30, 420) for _ in range(rng.randint(1, 6))]) for _ in range(rng.randint(1, 5))]
    lines = [str(len(runners))]
    lines += [f"{who} {len(laps)} {' '.join(laps)}" for who, laps in runners]
    return "\n".join(lines) + "\n"


if __name__ == "__main__":
    import sys

    args = sys.argv[1:]
    sys.stdout.write(generate(int(args[0]) if args else 1, args[1] if len(args) > 1 else "small"))
