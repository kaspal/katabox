"""Repair-shop clients (an SQL problem, ADR 0031)."""
import random

FIRST = ["Ana", "Bruno", "Clara", "Dario", "Elena", "Felix", "Gala", "Hugo"]
LAST = ["Rios", "Soto", "Vega", "Mora", "Paz", "Lara"]
EXAMPLES = {
    1: [[1, "Ana", "Rios", "555-0101", "ana@rios.test", "email"],
        [2, "Bruno", "Soto", "555-0102", None, "email"],
        [3, "Clara", "Vega", None, "clara@vega.test", None],
        [4, "Dario", "Mora", None, None, "phone"]],
    2: [[8, "Elena", "Paz", "555-0199", "elena@paz.test", None]],
    3: [[5, "Hugo", "Lara", None, "hugo@lara.test", "phone"],
        [6, "Gala", "Lara", "555-0150", None, None],
        [7, "Gala", "Lara", "555-0151", "gala@lara.test", "email"]],
}


def generate(seed, size):
    if size == "example":
        return {"tables": {"clients": EXAMPLES[seed]}}
    rng = random.Random(seed)
    n = 300 if size == "max" else rng.randint(1, 8)
    clients = []
    for cid in rng.sample(range(1, 5000), n):
        first, last = rng.choice(FIRST), rng.choice(LAST)
        phone = None if rng.random() < 0.35 else f"555-{rng.randint(0, 9999):04d}"
        email = None if rng.random() < 0.35 else f"{first.lower()}@{last.lower()}.test"
        clients.append([cid, first, last, phone, email, rng.choice(["phone", "email", None])])
    return {"tables": {"clients": clients}}
