from itertools import combinations


def best_programme(talks: list[list[int]]) -> int:
    # Obviously correct for a few talks: every set of talks that do not overlap.
    best = 0
    for k in range(1, len(talks) + 1):
        for group in combinations(sorted(talks, key=lambda t: t[0]), k):
            if all(group[i][1] <= group[i + 1][0] for i in range(k - 1)):
                best = max(best, sum(t[2] for t in group))
    return best
