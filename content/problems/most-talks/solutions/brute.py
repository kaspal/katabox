from itertools import combinations


def most_talks(talks: list[list[int]]) -> int:
    # Obviously correct (few talks): the largest set of pairwise compatible talks.
    for k in range(len(talks), 0, -1):
        for chosen in combinations(talks, k):
            ordered = sorted(chosen)
            if all(a[1] <= b[0] for a, b in zip(ordered, ordered[1:])):
                return k
    return 0
