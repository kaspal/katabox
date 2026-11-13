def trip_totals(miles: list[int], trips: list[list[int]]) -> list[int]:
    # Obviously correct: add up every trip's days.
    return [sum(miles[a : b + 1]) for a, b in trips]
