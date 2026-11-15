"""Stationery sales (an SQL problem, ADR 0031)."""
import random

CATEGORIES = ["paper", "pens", "ink", "cards", "folders", "tape", "glue"]
EXAMPLES = {
    1: [[1, "pens", 300], [2, "paper", 500], [3, "pens", 200], [4, "ink", 1000]],
    2: [[5, "cards", 999]],
    3: [[6, "tape", 100], [7, "glue", 100], [8, "folders", 100]],
}


def generate(seed, size):
    if size == "example":
        return {"tables": {"sales": EXAMPLES[seed]}}
    rng = random.Random(seed)
    n = 3000 if size == "max" else rng.randint(1, 10)
    cats = CATEGORIES if size == "max" else rng.sample(CATEGORIES, rng.randint(1, 4))
    sales = [[sid, rng.choice(cats), rng.choice([100, 200, 300]) if rng.random() < 0.4 else rng.randint(1, 5000)]
             for sid in rng.sample(range(1, 100000), n)]
    return {"tables": {"sales": sales}}
