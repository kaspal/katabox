"""Students and enrolments (an SQL problem, ADR 0031)."""
import random

NAMES = ["ana", "bo", "cy", "di", "ed", "fay", "gus", "ines"]
LETTERS = "abcdefghijklmnopqrstuvwxyz"


def names(rng, n):
    """n distinct names: the short list first, then made-up ones."""
    out = rng.sample(NAMES, min(n, len(NAMES)))
    while len(out) < n:
        w = "".join(rng.choice(LETTERS) for _ in range(rng.randint(3, 7)))
        if w not in out:
            out.append(w)
    return out
COURSES = ["ENG101", "ENG201", "FRA101", "GER101", "ITA101", "JPN101"]
EXAMPLES = {
    1: {"students": [[1, "ana"], [2, "bo"], [3, "cy"]],
        "enrollments": [[1, 1, "ENG101"], [2, 2, "ENG101"], [3, 1, "FRA101"], [4, 2, "FRA101"],
                        [5, 3, "GER101"]]},
    2: {"students": [[4, "di"], [5, "ed"]], "enrollments": [[6, 4, "ITA101"], [7, 5, "JPN101"]]},
    3: {"students": [[9, "fay"], [2, "gus"], [5, "hal"]],
        "enrollments": [[8, 9, "ENG201"], [9, 2, "ENG201"], [10, 5, "ENG201"]]},
}


def generate(seed, size):
    if size == "example":
        return {"tables": EXAMPLES[seed]}
    rng = random.Random(seed)
    n, courses, per = (150, COURSES, 4) if size == "max" else (rng.randint(1, 6), COURSES[:rng.randint(1, 4)], 3)
    sids = rng.sample(range(1, 1000), n)
    students = [[s, w] for s, w in zip(sids, names(rng, n))]
    pairs = set()
    for s in sids:
        for c in rng.sample(courses, rng.randint(0, min(per, len(courses)))):
            pairs.add((s, c))
    pairs = sorted(pairs)
    rng.shuffle(pairs)
    eids = rng.sample(range(1, 100000), len(pairs))
    return {"tables": {"students": students,
                       "enrollments": [[e, s, c] for e, (s, c) in zip(eids, pairs)]}}
