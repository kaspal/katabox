from itertools import combinations


def mirror_cuts(text: str) -> int:
    # Obviously correct (short texts): try every set of cut positions, fewest first.
    n = len(text)
    for k in range(n):
        for cut_at in combinations(range(1, n), k):
            bounds = [0, *cut_at, n]
            if all(text[a:b] == text[a:b][::-1] for a, b in zip(bounds, bounds[1:])):
                return k
    return n - 1
