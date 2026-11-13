def validate(inp):
    b = inp["bookings"]
    if not isinstance(b, list) or not 1 <= len(b) <= 10**4:
        return "1 ≤ bookings.length ≤ 10⁴"
    for r in b:
        if not isinstance(r, list) or len(r) != 2 or not all(type(v) is int for v in r) or not 0 <= r[0] <= r[1] <= 10**6:
            return "0 ≤ start ≤ end ≤ 10⁶"
    return None
