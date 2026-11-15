"""A bank statement and the shop's books (an SQL problem, ADR 0031)."""
import random

EXAMPLES = {
    1: {"bank": [[1, "P0001", 1500], [2, "P0002", 2000], [3, "P0004", 700]],
        "books": [[1, "P0001", 1500], [2, "P0002", 2500], [3, "P0003", 900]]},
    2: {"bank": [[1, "P0100", 4200]], "books": [[9, "P0100", 4200]]},
    3: {"bank": [], "books": [[1, "P0007", -300], [2, "P0005", 120]]},
}


def generate(seed, size):
    if size == "example":
        return {"tables": EXAMPLES[seed]}
    rng = random.Random(seed)
    n = 2000 if size == "max" else rng.randint(1, 8)
    refs = [f"P{r:04d}" for r in rng.sample(range(0, 10000), n)]
    bank, books = [], []
    for ref in refs:
        amount = rng.randint(-5000, 50000)
        r = rng.random()
        if r < 0.15:
            bank.append(ref_row(rng, ref, amount))
        elif r < 0.3:
            books.append(ref_row(rng, ref, amount))
        else:
            bank.append(ref_row(rng, ref, amount))
            other = amount + rng.choice([-1, 1, 100]) if rng.random() < 0.3 else amount
            books.append(ref_row(rng, ref, other))
    for table in (bank, books):
        ids = rng.sample(range(1, 100000), len(table))
        for row, i in zip(table, ids):
            row[0] = i
        rng.shuffle(table)
    return {"tables": {"bank": bank, "books": books}}


def ref_row(rng, ref, amount):
    return [0, ref, amount]
