from itertools import combinations


def smallest_squad(skills: int, people: list[int]) -> int:
    # Correct but exponential: try every team, smallest first.
    full = (1 << skills) - 1
    covered = 0
    for p in people:
        covered |= p
    if covered != full:
        return -1
    for k in range(1, len(people) + 1):
        for team in combinations(people, k):
            mask = 0
            for p in team:
                mask |= p
            if mask == full:
                return k
    return -1
