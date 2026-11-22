"""booking-rooms--f1 constraints."""


def validate(inp):
    m = inp["meetings"]
    if not isinstance(m, list) or not 1 <= len(m) <= 2 * 10**4:
        return "1 ≤ meetings.length ≤ 2·10⁴"
    for x in m:
        if not (isinstance(x, list) and len(x) == 2 and all(type(v) is int for v in x) and 0 <= x[0] < x[1] <= 10**9):
            return "meetings[i] = [start, end] with 0 ≤ start < end ≤ 10⁹"
    return None
