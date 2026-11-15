"""Furniture sales by region (an SQL problem, ADR 0031)."""
import random

REGIONS = ["North", "South", "East", "West", "Centre", "Coast"]
EXAMPLES = {
    1: [[1, "North", 5000], [2, "North", 9000], [3, "North", 7000], [4, "South", 3000],
        [5, "South", 3000], [6, "South", 1000]],
    2: [[7, "East", 2500], [8, "East", 2500]],
    3: [[9, "West", 800], [10, "Coast", 400], [11, "Coast", 600], [12, "Coast", 600]],
}


def generate(seed, size):
    if size == "example":
        return {"tables": {"sales": EXAMPLES[seed]}}
    rng = random.Random(seed)
    n = 2000 if size == "max" else rng.randint(1, 10)
    regions = REGIONS if size == "max" else rng.sample(REGIONS, rng.randint(1, 3))
    # Few distinct amounts, so ties at the top are common.
    pool = [rng.randint(1, 50) * 100 for _ in range(4 if size != "max" else 400)]
    sales = [[sid, rng.choice(regions), rng.choice(pool)] for sid in rng.sample(range(1, 100000), n)]
    return {"tables": {"sales": sales}}
