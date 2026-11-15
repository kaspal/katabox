"""Ration prices and team budgets as stdin text (a program problem, ADR 0028)."""
import random


def render(prices, budgets):
    lines = [f"{len(prices)} {len(budgets)}", " ".join(map(str, prices))]
    lines += map(str, budgets)
    return "\n".join(lines) + "\n"


def sized(rng, n):
    """Complexity profile input (ADR 0024): n prices and n budgets, mostly near the total."""
    prices = [rng.randint(1, 999) for _ in range(n)]
    total = sum(prices)
    return render(prices, [rng.randint(total * 9 // 10, total) for _ in range(n)])


def generate(seed, size):
    rng = random.Random(seed)
    if isinstance(size, int):
        return sized(rng, size)
    if size == "max":
        # 5·10⁴ short prices and (seed 1) 5·10⁴ budgets near the total: walking the sorted
        # list for every budget is about 2.4·10⁹ steps.
        prices = [rng.randint(1, 999) for _ in range(50_000)]
        total = sum(prices)
        if seed % 2 == 1:
            budgets = [rng.randint(total * 95 // 100, total + 1000) for _ in range(50_000)]
        else:
            # 5000 budgets anywhere from nothing to more than everything costs.
            budgets = [rng.randint(0, total + 1000) for _ in range(5_000)]
    elif seed % 5 == 0:
        # Edge shapes: a zero budget, budgets just below, at and above a prefix total,
        # the largest prices and budgets.
        n = rng.randint(1, 6)
        prices = [rng.choice([1, 1_000_000_000]) for _ in range(n)]
        cut = sorted(prices)
        totals = [sum(cut[: i + 1]) for i in range(n)]
        budgets = [0, 50_000_000_000_000]
        for t in rng.sample(totals, min(2, n)):
            budgets += [t - 1, t, t + 1]
        rng.shuffle(budgets)
    else:
        n = rng.randint(1, 8)
        prices = [rng.randint(1, 20) for _ in range(n)]
        budgets = [rng.randint(0, 25 * n // 2 + 5) for _ in range(rng.randint(1, 6))]
    return render(prices, budgets)


if __name__ == "__main__":
    import sys

    args = sys.argv[1:]
    sys.stdout.write(generate(int(args[0]) if args else 1, args[1] if len(args) > 1 else "small"))
