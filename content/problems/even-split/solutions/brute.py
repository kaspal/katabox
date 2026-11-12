from fractions import Fraction


def settle(paid: list[float]) -> list[float]:
    # Obviously correct: exact rational arithmetic, rounded to a float at the end.
    exact = [Fraction(p) for p in paid]
    share = sum(exact) / len(exact)
    return [float(p - share) for p in exact]
