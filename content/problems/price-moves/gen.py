"""Closing prices (an SQL problem, ADR 0031)."""
import random
from datetime import date, timedelta

TICKERS = ["ACME", "BOLT", "CORA", "DYNE", "EKO", "FIG"]
EXAMPLES = {
    1: [[1, "ACME", "2026-03-02", 1000], [2, "ACME", "2026-03-03", 1050], [3, "ACME", "2026-03-04", 1029],
        [4, "BOLT", "2026-03-02", 400], [5, "BOLT", "2026-03-04", 300]],
    2: [[6, "CORA", "2026-03-02", 777]],
    3: [[9, "DYNE", "2026-04-02", 3], [7, "DYNE", "2026-04-01", 8], [8, "EKO", "2026-04-01", 250],
        [10, "EKO", "2026-04-02", 250]],
}


def generate(seed, size):
    if size == "example":
        return {"tables": {"prices": EXAMPLES[seed]}}
    rng = random.Random(seed)
    tickers = TICKERS if size == "max" else rng.sample(TICKERS, rng.randint(1, 3))
    span = 500 if size == "max" else 6
    prices = []
    for t in tickers:
        days = sorted(rng.sample(range(span), rng.randint(0, span if size != "max" else 300)))
        price = rng.randint(50, 5000)
        for d in days:
            price = max(1, price + rng.randint(-price // 10 - 1, price // 10 + 1))
            prices.append([t, (date(2026, 1, 1) + timedelta(days=d % 365)).isoformat(), price])
    # One close per ticker and day (days past 365 wrap; keep the first).
    seen, unique = set(), []
    for t, day, p in prices:
        if (t, day) not in seen:
            seen.add((t, day))
            unique.append([t, day, p])
    rng.shuffle(unique)
    ids = rng.sample(range(1, 100000), len(unique))
    return {"tables": {"prices": [[i] + row for i, row in zip(ids, unique)]}}
