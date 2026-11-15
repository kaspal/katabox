"""Bike-share trips (an SQL problem, ADR 0031)."""
import random

STATIONS = ["Harbour", "Market", "Museum", "Park", "Station", "Bridge", "Campus"]
EXAMPLES = {
    1: [[1, "Market", "Park", 12], [2, "Park", "Market", 15], [3, "Museum", "Museum", 40],
        [4, "Market", "Museum", 9], [5, "Museum", "Park", 20]],
    2: [[6, "Bridge", "Bridge", 30], [7, "Bridge", "Bridge", 25]],
    3: [[8, "Campus", "Harbour", 7], [9, "Harbour", "Campus", 8], [10, "Campus", "Harbour", 6],
        [11, "Harbour", "Harbour", 50]],
}


def generate(seed, size):
    if size == "example":
        return {"tables": {"trips": EXAMPLES[seed]}}
    rng = random.Random(seed)
    n = 3000 if size == "max" else rng.randint(0, 10)
    stations = STATIONS if size == "max" else rng.sample(STATIONS, rng.randint(1, 4))
    trips = []
    for tid in rng.sample(range(1, 100000), n):
        a = rng.choice(stations)
        b = a if rng.random() < 0.25 else rng.choice(stations)
        trips.append([tid, a, b, rng.randint(1, 90)])
    return {"tables": {"trips": trips}}
