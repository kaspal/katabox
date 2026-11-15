"""Newsletter readers (an SQL problem, ADR 0031)."""
import random

CITIES = ["Lima", "Quito", "Cusco", "Bogota", "Cali"]
NAMES = ["ana", "bo", "cy", "di", "ed", "fay", "gus", "max", "amy", "lu"]
EXAMPLES = {
    1: [[1, "ana", "ana@mail.test", "Lima"], [2, "bo", None, "Quito"], [3, "cy", "", "Lima"],
        [4, "di", "   ", "Bogota"], [5, "ed", "ed@mail.test", "Quito"]],
    2: [[7, "fay", "fay@mail.test", "Cusco"]],
    3: [[9, "max", None, "Lima"], [3, "max", "", "Lima"], [5, "amy", None, "Lima"]],
}


def email(rng, name):
    r = rng.random()
    if r < 0.25:
        return None
    if r < 0.4:
        return rng.choice(["", " ", "   "])
    return name + "@mail.test"


def generate(seed, size):
    if size == "example":
        return {"tables": {"readers": EXAMPLES[seed]}}
    rng = random.Random(seed)
    n = 300 if size == "max" else rng.randint(1, 8)
    ids = rng.sample(range(1, 2000), n)
    readers = []
    for rid in ids:
        name = rng.choice(NAMES)
        readers.append([rid, name, email(rng, name), rng.choice(CITIES)])
    return {"tables": {"readers": readers}}
