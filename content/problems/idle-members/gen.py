"""Gym members and visits (an SQL problem, ADR 0031)."""
import random
from datetime import date, timedelta

NAMES = ["ana", "bo", "cy", "di", "ed", "fay", "gus", "ines", "jo", "kai"]
EXAMPLES = {
    1: {"members": [[1, "ana"], [2, "bo"], [3, "cy"]],
        "visits": [[1, 1, "2026-11-03"], [2, 2, "2026-10-30"], [3, 1, "2026-11-20"]]},
    2: {"members": [[4, "di"]], "visits": [[5, 4, "2026-11-30"]]},
    3: {"members": [[9, "ed"], [6, "ed"], [7, "ana"]], "visits": [[8, 7, "2026-12-01"]]},
}


def generate(seed, size):
    if size == "example":
        return {"tables": EXAMPLES[seed]}
    rng = random.Random(seed)
    n_m, n_v = (300, 3000) if size == "max" else (rng.randint(1, 6), rng.randint(0, 10))
    mids = rng.sample(range(1, 1000), n_m)
    members = [[m, rng.choice(NAMES)] for m in mids]
    visits = []
    for vid in rng.sample(range(1, 10000), n_v):
        # October to December, with the month's edges often.
        day = rng.choice([date(2026, 10, 31), date(2026, 11, 1), date(2026, 11, 30), date(2026, 12, 1)]) \
            if rng.random() < 0.3 else date(2026, 10, 1) + timedelta(days=rng.randint(0, 91))
        visits.append([vid, rng.choice(mids), day.isoformat()])
    return {"tables": {"members": members, "visits": visits}}
