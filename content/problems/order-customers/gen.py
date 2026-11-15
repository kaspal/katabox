"""Bakery customers and orders (an SQL problem, ADR 0031)."""
import random
from datetime import date, timedelta

NAMES = ["Ana", "Bruno", "Clara", "Dario", "Elena", "Felix", "Gala"]
CITIES = ["Porto", "Braga", "Faro", "Evora"]
EXAMPLES = {
    1: {"customers": [[1, "Ana", "Porto"], [2, "Bruno", "Braga"], [3, "Clara", "Faro"]],
        "orders": [[1, 2, "2026-03-01", 12500], [2, 3, "2026-03-01", 9999], [3, 1, "2026-03-02", 10000],
                   [4, 2, "2026-03-03", 2500]]},
    2: {"customers": [[5, "Elena", "Evora"]], "orders": [[7, 5, "2026-04-10", 800]]},
    3: {"customers": [[2, "Felix", "Porto"], [1, "Gala", "Faro"]],
        "orders": [[1, 2, "2026-05-01", 30000], [2, 1, "2026-05-02", 30000], [3, 2, "2026-05-02", 45000]]},
}


def generate(seed, size):
    if size == "example":
        return {"tables": EXAMPLES[seed]}
    rng = random.Random(seed)
    n_c, n_o = (200, 2000) if size == "max" else (rng.randint(1, 5), rng.randint(0, 10))
    # Ids from small ranges, so customer and order ids overlap.
    cids = rng.sample(range(1, 400), n_c)
    customers = [[c, rng.choice(NAMES), rng.choice(CITIES)] for c in cids]
    orders = []
    for oid in rng.sample(range(1, 4000), n_o):
        total = rng.choice([9999, 10000, 10001]) if rng.random() < 0.3 else rng.randint(500, 40000)
        day = (date(2026, 1, 1) + timedelta(days=rng.randint(0, 364))).isoformat()
        orders.append([oid, rng.choice(cids), day, total])
    return {"tables": {"customers": customers, "orders": orders}}
