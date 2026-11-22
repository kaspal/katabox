"""merge-bookings--f1 constraints."""


def validate(inp):
    for name in ("roomA", "roomB"):
        sheet = inp[name]
        if not isinstance(sheet, list) or not 1 <= len(sheet) <= 10**4:
            return f"1 ≤ {name}.length ≤ 10⁴"
        for b in sheet:
            if not (isinstance(b, list) and len(b) == 2 and all(type(x) is int for x in b) and 0 <= b[0] <= b[1] <= 10**6):
                return "each booking is [start, end] with 0 ≤ start ≤ end ≤ 10⁶"
    return None
