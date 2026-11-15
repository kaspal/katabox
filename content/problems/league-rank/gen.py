"""League tables (an SQL problem, ADR 0031)."""
import random

NAMES = ["Rovers", "United", "Albion", "Wanderers", "Rangers", "Athletic", "Town", "City", "Harriers"]
EXAMPLES = {
    1: [[1, "Rovers", "A", 30], [2, "United", "A", 27], [3, "Albion", "A", 30], [4, "Town", "A", 25],
        [5, "City", "B", 12]],
    2: [[6, "Rangers", "C", 0]],
    3: [[7, "Athletic", "B", 9], [8, "Harriers", "B", 9], [9, "Wanderers", "B", 9], [10, "Town", "B", 4]],
}


def generate(seed, size):
    if size == "example":
        return {"tables": {"teams": EXAMPLES[seed]}}
    rng = random.Random(seed)
    n = 2000 if size == "max" else rng.randint(1, 9)
    divs = ["A", "B", "C", "D"] if size == "max" else rng.sample(["A", "B", "C", "D"], rng.randint(1, 2))
    lo, hi = (0, 300) if size == "max" else (0, 6)  # small ranges: many ties
    teams = [[tid, rng.choice(NAMES), rng.choice(divs), rng.randint(lo, hi)]
             for tid in rng.sample(range(1, 100000), n)]
    return {"tables": {"teams": teams}}
