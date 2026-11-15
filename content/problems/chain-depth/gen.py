"""A hospital's reporting lines (an SQL problem, ADR 0031)."""
import random

NAMES = ["Ada", "Bram", "Cleo", "Dev", "Eva", "Finn", "Gwen", "Hal", "Iris", "Jon"]
EXAMPLES = {
    1: [[1, "Iris", None], [2, "Bram", 1], [3, "Cleo", 1], [4, "Dev", 3], [5, "Ada", 4]],
    2: [[9, "Eva", None]],
    3: [[7, "Finn", 8], [8, "Gwen", None], [6, "Finn", 8], [5, "Hal", 6]],
}


def generate(seed, size):
    if size == "example":
        return {"tables": {"staff": EXAMPLES[seed]}}
    rng = random.Random(seed)
    n = 1000 if size == "max" else rng.randint(1, 8)
    ids = rng.sample(range(1, 10000), n)
    staff = [[ids[0], rng.choice(NAMES), None]]
    for i in range(1, n):
        # A long chain on the max test (seed 1), a bushy tree otherwise.
        b = ids[i - 1] if size == "max" and seed == 1 else rng.choice(ids[:i])
        staff.append([ids[i], rng.choice(NAMES), b])
    rng.shuffle(staff)
    return {"tables": {"staff": staff}}
