"""Sign-ins (an SQL problem, ADR 0031)."""
import random
from datetime import date, timedelta

USERS = ["ana_r", "bo99", "cyrus", "dina", "eli_k", "fay"]
EXAMPLES = {
    1: [[1, "ana_r", "2026-01-01"], [2, "ana_r", "2026-01-02"], [3, "ana_r", "2026-01-02"], [4, "ana_r", "2026-01-03"],
        [5, "ana_r", "2026-01-05"], [6, "bo99", "2026-01-01"], [7, "bo99", "2026-01-03"]],
    2: [[8, "cyrus", "2025-12-31"], [9, "cyrus", "2026-01-01"]],
    3: [[10, "dina", "2026-02-27"], [11, "dina", "2026-02-28"], [12, "dina", "2026-03-01"], [13, "eli_k", "2026-03-01"],
        [14, "eli_k", "2026-03-01"], [15, "eli_k", "2026-03-02"], [16, "eli_k", "2026-03-04"], [17, "eli_k", "2026-03-05"],
        [18, "eli_k", "2026-03-06"]],
}


def generate(seed, size):
    if size == "example":
        return {"tables": {"logins": EXAMPLES[seed]}}
    rng = random.Random(seed)
    n, span = (3000, 370) if size == "max" else (rng.randint(0, 14), 8)
    users = USERS if size == "max" else rng.sample(USERS, rng.randint(1, 3))
    start = date(2025, 12, 20)
    logins = [[lid, rng.choice(users), (start + timedelta(days=rng.randrange(span))).isoformat()]
              for lid in rng.sample(range(1, 100000), n)]
    return {"tables": {"logins": logins}}
