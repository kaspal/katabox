"""A ferry network (an SQL problem, ADR 0031)."""
import random

EXAMPLES = {
    1: [[1, "Central", "Harbour"], [2, "Harbour", "Island"], [3, "Island", "Lighthouse"],
        [4, "Lighthouse", "Cove"], [5, "Central", "Island"], [6, "Island", "Central"]],
    2: [[7, "Market", "Central"]],
    3: [[8, "Central", "Dock"], [9, "Dock", "Bay"], [10, "Bay", "Dock"], [11, "Dock", "Reef"],
        [12, "Reef", "Cape"], [13, "Cape", "Point"]],
}
WORDS = ["Harbour", "Island", "Lighthouse", "Cove", "Market", "Dock", "Bay", "Reef", "Cape", "Point",
         "Quay", "Strand", "Abbey", "Mill", "Fort", "Ness", "Rock", "Sands"]


def generate(seed, size):
    if size == "example":
        return {"tables": {"routes": EXAMPLES[seed]}}
    rng = random.Random(seed)
    if size == "max":
        piers = ["Central"] + [w + s for w in WORDS for s in ["", "ton", "ham"]][:59]
        n = 400
    else:
        piers = ["Central"] + rng.sample(WORDS, rng.randint(2, 7))
        n = rng.randint(1, 12)
    pairs = set()
    while len(pairs) < n:
        a, b = rng.sample(piers, 2)
        pairs.add((a, b))
        if len(pairs) >= len(piers) * (len(piers) - 1):
            break
    pairs = sorted(pairs)
    rng.shuffle(pairs)
    ids = rng.sample(range(1, 100000), len(pairs))
    return {"tables": {"routes": [[i, a, b] for i, (a, b) in zip(ids, pairs)]}}
