"""Orchard sales (an SQL problem, ADR 0031)."""
import random
from datetime import date, timedelta

EXAMPLES = {
    1: [[1, "2025-01-15", 1200], [2, "2025-03-31", 800], [3, "2025-04-01", 500], [4, "2025-12-24", 3000],
        [5, "2026-07-04", 2200]],
    2: [],
    3: [[6, "2024-06-30", 100], [7, "2024-09-01", 200], [8, "2024-09-30", 300], [9, "2024-10-01", 400]],
}
EDGES = ["03-31", "04-01", "06-30", "07-01", "09-30", "10-01", "12-31", "01-01"]


def generate(seed, size):
    if size == "example":
        return {"tables": {"sales": EXAMPLES[seed]}}
    rng = random.Random(seed)
    n = 3000 if size == "max" else rng.randint(0, 10)
    years = list(range(2020, 2027)) if size == "max" else rng.sample(range(2020, 2027), rng.randint(1, 2))
    sales = []
    for sid in rng.sample(range(1, 100000), n):
        y = rng.choice(years)
        if rng.random() < 0.4:
            day = f"{y}-{rng.choice(EDGES)}"
        else:
            day = (date(y, 1, 1) + timedelta(days=rng.randint(0, 364))).isoformat()
        sales.append([sid, day, rng.randint(100, 5000)])
    return {"tables": {"sales": sales}}
