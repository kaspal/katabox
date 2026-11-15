"""Food-truck orders (an SQL problem, ADR 0031)."""
import random
from datetime import datetime, timedelta

EXAMPLES = {
    1: [[1, "2026-05-01 12:05", 850], [2, "2026-05-01 12:05", 1200], [3, "2026-05-01 19:40", 640],
        [4, "2026-05-03 13:10", 990]],
    2: [],
    3: [[7, "2026-05-09 23:59", 500], [8, "2026-05-10 00:00", 700]],
}


def generate(seed, size):
    if size == "example":
        return {"tables": {"orders": EXAMPLES[seed]}}
    rng = random.Random(seed)
    n, days = (3000, 120) if size == "max" else (rng.randint(0, 12), 3)
    start = datetime(2026, 5, 1)
    orders = []
    for oid in rng.sample(range(1, 100000), n):
        # Several orders share a minute; times run up to midnight.
        t = start + timedelta(days=rng.randrange(days), minutes=rng.choice([725, 726, 1439, 0]) if rng.random() < 0.3 else rng.randrange(1440))
        orders.append([oid, t.strftime("%Y-%m-%d %H:%M"), rng.randint(300, 4000)])
    return {"tables": {"orders": orders}}
