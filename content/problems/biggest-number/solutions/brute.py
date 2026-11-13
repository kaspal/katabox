from itertools import permutations


def biggest_number(cards: list[int]) -> str:
    # Obviously correct (few cards): try every order; all results have the same length.
    best = max("".join(map(str, p)) for p in permutations(cards))
    return best.lstrip("0") or "0"
