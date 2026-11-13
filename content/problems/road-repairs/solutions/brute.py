def road_repairs(length: int, repairs: list[list[int]]) -> list[int]:
    # Obviously correct: count each section's repairs.
    return [sum(c for a, b, c in repairs if a <= i <= b) for i in range(length)]
