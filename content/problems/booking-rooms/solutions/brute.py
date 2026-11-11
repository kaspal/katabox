def min_rooms(meetings: list[list[int]]) -> int:
    # Obviously correct: the busiest moment is the start of some meeting; count the
    # meetings in progress at each start.
    return max(sum(1 for a, b in meetings if a <= s < b) for s, _ in meetings)
