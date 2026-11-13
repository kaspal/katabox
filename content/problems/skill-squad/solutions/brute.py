from itertools import combinations


def smallest_squad(skills: int, people: list[int]) -> int:
    # Obviously correct (few volunteers): every team, smallest first.
    full = (1 << skills) - 1
    for k in range(1, len(people) + 1):
        for team in combinations(people, k):
            covered = 0
            for p in team:
                covered |= p
            if covered == full:
                return k
    return -1
