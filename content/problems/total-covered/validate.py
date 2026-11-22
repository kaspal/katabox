"""total-covered constraints."""


def validate(inp):
    shifts = inp["shifts"]
    if not isinstance(shifts, list) or not 1 <= len(shifts) <= 10**4:
        return "1 ≤ shifts.length ≤ 10⁴"
    for sh in shifts:
        if not isinstance(sh, list) or len(sh) != 2 or not all(type(x) is int for x in sh):
            return "each shift is [start, end]"
        if not 0 <= sh[0] < sh[1] <= 10**6:
            return "0 ≤ start < end ≤ 10⁶"
    return None
