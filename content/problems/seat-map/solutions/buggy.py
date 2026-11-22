def seat_map(rows: int, cols: int, bookings: list[list[int]]) -> list[str]:
    hall = [["."] * cols] * rows
    for r, c in bookings:
        hall[r][c] = "X"
    return ["".join(row) for row in hall]
