"""Shop products (an SQL problem, ADR 0031)."""
import random

WORDS = ["lamp", "rug", "vase", "mug", "clock", "tray", "pen", "ink", "pad", "sofa", "cup", "bowl"]
EDGES = [1999, 2000, 2001, 4998, 4999, 5000]
EXAMPLES = {
    1: [[1, "lamp", 2000, 1], [2, "rug", 4999, 1], [3, "vase", 5000, 1], [4, "mug", 1999, 1],
        [5, "clock", 3500, 0], [6, "tray", 3500, 1]],
    2: [[1, "pen", 2500, 1], [2, "ink", 2500, 1], [3, "pad", 2500, 1]],
    3: [[1, "sofa", 120000, 1], [2, "cup", 450, 1]],
}


def generate(seed, size):
    if size == "example":
        return {"tables": {"products": EXAMPLES[seed]}}
    rng = random.Random(seed)
    n = 500 if size == "max" else rng.randint(1, 10)
    products = []
    for pid in rng.sample(range(1, 5000), n):
        name = rng.choice(WORDS) if rng.random() < 0.7 else rng.choice(WORDS) + " " + rng.choice(WORDS)
        price = rng.choice(EDGES) if rng.random() < 0.4 else rng.randint(100, 9999)
        products.append([pid, name, price, 1 if rng.random() < 0.75 else 0])
    return {"tables": {"products": products}}
