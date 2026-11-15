"""A hand-typed flight log (an SQL problem, ADR 0031)."""
import random

CITIES = ["Lima", "Quito", "Cusco", "Bogota", "Cali", "Santiago", "Arica"]
EXAMPLES = {
    1: [[1, "Lima", "Quito"], [2, "quito", "LIMA"], [3, " Cusco", "Lima "], [4, "Lima", "Cusco"]],
    2: [[5, "Cali", "cali"], [6, "Arica", "  Arica"]],
    3: [[7, "Bogota", "Cali"], [8, "Santiago", "Arica"], [9, "CALI", "bogota"], [10, "arica", "Bogota"]],
}


def typed(rng, city):
    r = rng.random()
    if r < 0.3:
        city = city.upper()
    elif r < 0.5:
        city = city.lower()
    return " " * rng.choice([0, 0, 1, 2]) + city + " " * rng.choice([0, 0, 1])


def generate(seed, size):
    if size == "example":
        return {"tables": {"flights": EXAMPLES[seed]}}
    rng = random.Random(seed)
    n = 2000 if size == "max" else rng.randint(1, 10)
    cities = CITIES if size == "max" else rng.sample(CITIES, rng.randint(2, 4))
    flights = []
    for fid in rng.sample(range(1, 10000), n):
        a = rng.choice(cities)
        b = a if rng.random() < 0.1 else rng.choice(cities)
        flights.append([fid, typed(rng, a), typed(rng, b)])
    return {"tables": {"flights": flights}}
