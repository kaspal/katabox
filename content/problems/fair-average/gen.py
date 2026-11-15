"""Board-game reviews (an SQL problem, ADR 0031)."""
import random

GAMES = ["Azul", "Catan", "Carcassonne", "Dixit", "Hive", "Patchwork", "Splendor"]
EXAMPLES = {
    1: [[1, "Hive", 5], [2, "Hive", None], [3, "Hive", 4], [4, "Dixit", 3], [5, "Dixit", 4],
        [6, "Dixit", 4], [7, "Azul", None]],
    2: [[8, "Catan", 2]],
    3: [[9, "Splendor", 1], [10, "Splendor", 2], [11, "Splendor", 2], [12, "Splendor", 2],
        [13, "Splendor", 3], [14, "Splendor", 3], [15, "Splendor", 3], [16, "Splendor", 1]],
}


def generate(seed, size):
    if size == "example":
        return {"tables": {"reviews": EXAMPLES[seed]}}
    rng = random.Random(seed)
    n = 3000 if size == "max" else rng.randint(1, 10)
    games = GAMES if size == "max" else rng.sample(GAMES, rng.randint(1, 3))
    reviews = [[rid, rng.choice(games), None if rng.random() < 0.25 else rng.randint(1, 5)]
               for rid in rng.sample(range(1, 100000), n)]
    return {"tables": {"reviews": reviews}}
