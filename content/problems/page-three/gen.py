"""Marketplace listings (an SQL problem, ADR 0031)."""
import random

WORDS = ["bike", "desk", "chair", "lamp", "guitar", "coat", "boots", "radio", "kettle", "tent"]


def listings(rng, n, hidden_share, ratings=(0, 5), id_range=10000):
    out = []
    for lid in rng.sample(range(1, id_range), n):
        title = " ".join(rng.choice(WORDS) for _ in range(rng.randint(1, 2)))
        out.append([lid, title, rng.randint(*ratings), 1 if rng.random() < hidden_share else 0])
    return out


def generate(seed, size):
    rng = random.Random(seed)
    if size == "example":
        # 1: 25 visible, page 3 has five; 2: 12 listings, no page 3; 3: hidden ones and ties.
        n, hidden, ratings = {1: (25, 0.0, (1, 5)), 2: (12, 0.0, (1, 5)), 3: (34, 0.15, (3, 4))}[seed]
        return {"tables": {"listings": listings(rng, n, hidden, ratings, 100)}}
    if size == "max":
        return {"tables": {"listings": listings(rng, 1000, 0.2)}}
    # Around the page boundaries: 18 to 34 listings, sometimes many ties.
    ratings = (4, 5) if seed % 3 == 0 else (0, 5)
    return {"tables": {"listings": listings(rng, rng.randint(18, 34), rng.choice([0.0, 0.1, 0.3]), ratings)}}
