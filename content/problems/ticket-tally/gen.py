"""Random ticket ledgers as stdin text (a program problem, ADR 0028)."""
import random


def generate(seed, size):
    rng = random.Random(seed)
    if size == "max":
        # 50 full days: 50 000 prices, the most the input may hold.
        days = [[rng.randint(1, 10_000) for _ in range(1000)] for _ in range(50)]
    elif seed % 5 == 0:
        # Edge shapes: empty days, single tickets, the top price.
        days = [[] if rng.random() < 0.5 else [rng.choice([1, 10_000])] for _ in range(rng.randint(1, 6))]
    else:
        days = [
            [rng.randint(1, 60) for _ in range(rng.randint(0, 6))]
            for _ in range(rng.randint(1, 5))
        ]
    lines = [str(len(days))]
    for prices in days:
        lines.append(str(len(prices)))
        lines.append(" ".join(map(str, prices)))
    return "\n".join(lines) + "\n"


if __name__ == "__main__":
    import sys

    args = sys.argv[1:]
    sys.stdout.write(generate(int(args[0]) if args else 1, args[1] if len(args) > 1 else "small"))
