"""Library members and loans (an SQL problem, ADR 0031): {"tables": {...}}."""
import random
from datetime import date, timedelta

WORDS = ["Dune", "Emma", "Ulysses", "Beloved", "Middlemarch", "Odyssey", "Iliad", "Persuasion",
         "Walden", "Hamlet", "Moby", "Dick", "Little", "Women", "Night", "Sea", "Rose", "Garden"]
START = date(2026, 1, 1)


def name(rng):
    return "".join(rng.choice("abcdefghijklmnopqrstuvwxyz") for _ in range(rng.randint(1, 10)))


def title(rng, pool):
    if pool and rng.random() < 0.3:
        return rng.choice(pool)  # the same book borrowed again
    return " ".join(rng.choice(WORDS) for _ in range(rng.randint(1, 3)))


def generate(seed, size):
    rng = random.Random(seed)
    if size == "max":
        n_members, n_loans, spread = 200, 2000, 20
    else:
        n_members, n_loans, spread = rng.randint(1, 5), rng.randint(0, 12), 4
    member_ids = rng.sample(range(1, 1000), n_members)
    members = [[mid, name(rng)] for mid in member_ids]
    loan_ids = rng.sample(range(1, 100000), n_loans)
    titles = []
    loans = []
    for lid in loan_ids:
        t = title(rng, titles)
        titles.append(t)
        due = START + timedelta(days=rng.randint(0, 300))
        r = rng.random()
        if r < 0.2:
            returned = None
        elif r < 0.4:
            returned = due  # on the due day: not late
        else:
            returned = max(START, due + timedelta(days=rng.randint(-spread, spread * 3)))
        loans.append([lid, rng.choice(member_ids), t, due.isoformat(),
                      returned.isoformat() if returned else None])
    return {"tables": {"members": members, "loans": loans}}


if __name__ == "__main__":
    import json
    import sys
    print(json.dumps(generate(int(sys.argv[1]), sys.argv[2] if len(sys.argv) > 2 else "small")))
