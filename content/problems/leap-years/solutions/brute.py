import calendar


def leap_years(start: int, end: int) -> int:
    # Obviously correct: the standard library's calendar knows the rule.
    return sum(1 for y in range(start, end + 1) if calendar.isleap(y))
