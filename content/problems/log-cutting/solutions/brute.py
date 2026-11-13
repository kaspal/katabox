from itertools import permutations


def cut_cost(length: int, marks: list[int]) -> int:
    # Obviously correct (few marks): try every order of cuts.
    best = None
    for order in permutations(marks):
        cuts, total = [0, length], 0
        for c in order:
            left = max(x for x in cuts if x < c)
            right = min(x for x in cuts if x > c)
            total += right - left
            cuts.append(c)
        best = total if best is None else min(best, total)
    return best
