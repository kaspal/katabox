def seat_map(rows: int, cols: int, bookings: list[list[int]]) -> list[str]:
    # Obviously correct: decide each seat on its own.
    booked = {(r, c) for r, c in bookings}
    return ["".join("X" if (r, c) in booked else "." for c in range(cols)) for r in range(rows)]
