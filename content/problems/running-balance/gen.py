"""Credit-union transactions (an SQL problem, ADR 0031)."""
import random
from datetime import date, timedelta

EXAMPLES = {
    1: [[1, "AC001", "2026-02-01", 10000], [2, "AC001", "2026-02-03", -2500], [3, "AC002", "2026-02-01", 500],
        [4, "AC001", "2026-02-03", -1000], [5, "AC002", "2026-02-02", 700]],
    2: [],
    3: [[9, "AC007", "2026-03-10", 300], [6, "AC007", "2026-03-10", 200], [7, "AC007", "2026-03-09", -50]],
}


def generate(seed, size):
    if size == "example":
        return {"tables": {"transactions": EXAMPLES[seed]}}
    rng = random.Random(seed)
    n, accounts, days = (3000, 40, 60) if size == "max" else (rng.randint(0, 10), rng.randint(1, 3), 3)
    accts = [f"AC{a:03d}" for a in rng.sample(range(1, 1000), accounts)]
    rows = []
    for tid in rng.sample(range(1, 100000), n):
        day = (date(2026, 1, 1) + timedelta(days=rng.randrange(days))).isoformat()
        amount = rng.choice([-1, 1]) * rng.randint(1, 50000)
        rows.append([tid, rng.choice(accts), day, amount])
    return {"tables": {"transactions": rows}}
