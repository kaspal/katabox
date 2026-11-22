"""seat-map constraints."""


def validate(inp):
    rows, cols, bs = inp["rows"], inp["cols"], inp["bookings"]
    if type(rows) is not int or type(cols) is not int or not (1 <= rows <= 300 and 1 <= cols <= 300):
        return "1 ≤ rows, cols ≤ 300"
    if not isinstance(bs, list) or len(bs) > 10**4:
        return "at most 10⁴ bookings"
    for b in bs:
        if not isinstance(b, list) or len(b) != 2 or not all(type(x) is int for x in b):
            return "each booking is [row, col]"
        if not (0 <= b[0] < rows and 0 <= b[1] < cols):
            return "0 ≤ row < rows, 0 ≤ col < cols"
    return None
