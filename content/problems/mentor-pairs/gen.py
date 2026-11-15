"""Newsroom staff and mentors (an SQL problem, ADR 0031)."""
import random

NAMES = ["Ada", "Bea", "Carl", "Dina", "Eli", "Fran", "Gil", "Hana", "Ivo", "Jen"]
EXAMPLES = {
    1: [[1, "Ada", None], [2, "Bea", 1], [3, "Carl", 1], [4, "Dina", 3]],
    2: [[5, "Eli", None]],
    3: [[7, "Fran", 9], [8, "Fran", None], [9, "Gil", 7]],
}


def generate(seed, size):
    if size == "example":
        return {"tables": {"staff": EXAMPLES[seed]}}
    rng = random.Random(seed)
    n = 500 if size == "max" else rng.randint(1, 8)
    ids = rng.sample(range(1, 5000), n)
    staff = []
    for sid in ids:
        others = [x for x in ids if x != sid]
        mentor = rng.choice(others) if others and rng.random() < 0.65 else None
        staff.append([sid, rng.choice(NAMES), mentor])
    return {"tables": {"staff": staff}}
