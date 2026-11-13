def has_ticket(seats: list[list[int]], ticket: int) -> bool:
    # Obviously correct: look at every seat.
    return any(ticket in row for row in seats)
