"""Airports, flights and bookings (an SQL problem, ADR 0031)."""
import random
from datetime import datetime, timedelta

AIRPORTS = [["LIM", "Lima"], ["CUZ", "Cusco"], ["UIO", "Quito"], ["BOG", "Bogota"],
            ["SCL", "Santiago"], ["AQP", "Arequipa"]]
NAMES = ["ana", "bo", "cy", "di", "ed", "fay"]
EXAMPLES = {
    1: {"airports": AIRPORTS[:3],
        "flights": [[1, "KB101", "LIM", "CUZ", "2026-06-01 08:00"], [2, "KB102", "CUZ", "LIM", "2026-06-05 18:30"],
                    [3, "KB200", "LIM", "UIO", "2026-06-02 07:15"]],
        "bookings": [[1, "bo", 3], [2, "ana", 2], [3, "ana", 1]]},
    2: {"airports": AIRPORTS[:2], "flights": [[4, "KB300", "CUZ", "LIM", "2026-07-01 12:00"]], "bookings": []},
    3: {"airports": [["BOG", "Bogota"], ["SCL", "Santiago"]],
        "flights": [[5, "KB410", "SCL", "BOG", "2026-08-01 09:00"], [6, "KB411", "BOG", "SCL", "2026-07-20 10:00"]],
        "bookings": [[4, "cy", 5], [7, "cy", 6]]},
}


def generate(seed, size):
    if size == "example":
        return {"tables": EXAMPLES[seed]}
    rng = random.Random(seed)
    airports = AIRPORTS if size == "max" else rng.sample(AIRPORTS, rng.randint(2, 4))
    codes = [c for c, _ in airports]
    n_f, n_b = (300, 2000) if size == "max" else (rng.randint(1, 5), rng.randint(0, 10))
    starts = rng.sample(range(0, 365 * 24 * 4), n_f)
    flights = []
    for fid, q in zip(rng.sample(range(1, 5000), n_f), starts):
        src, dst = rng.sample(codes, 2)
        dep = (datetime(2026, 1, 1) + timedelta(minutes=15 * q)).strftime("%Y-%m-%d %H:%M")
        flights.append([fid, f"KB{rng.randint(100, 999)}", src, dst, dep])
    fids = [f[0] for f in flights]
    bookings = [[b, rng.choice(NAMES), rng.choice(fids)] for b in rng.sample(range(1, 100000), n_b)]
    return {"tables": {"airports": airports, "flights": flights, "bookings": bookings}}
