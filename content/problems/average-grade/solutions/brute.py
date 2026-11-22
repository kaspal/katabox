from fractions import Fraction


def average_grade(grades: list[int]) -> float:
    # Obviously correct: exact arithmetic, then a float.
    return float(Fraction(sum(grades), len(grades)))
