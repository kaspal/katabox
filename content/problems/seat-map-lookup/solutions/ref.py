def has_ticket(seats: list[list[int]], ticket: int) -> bool:
    r, c = 0, len(seats[0]) - 1
    while r < len(seats) and c >= 0:
        v = seats[r][c]
        if v == ticket:
            return True
        if v > ticket:
            c -= 1
        else:
            r += 1
    return False
