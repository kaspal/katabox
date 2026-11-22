"""frozen-ponds--f1 constraints."""


def validate(inp):
    ice = inp["ice"]
    if not isinstance(ice, list) or not 1 <= len(ice) <= 400 or not all(isinstance(r, str) for r in ice):
        return "1 ≤ ice.length ≤ 400"
    cols = len(ice[0])
    if not 1 <= cols <= 400 or any(len(r) != cols for r in ice):
        return "1 ≤ ice[i].length ≤ 400, all rows the same length"
    if any(c not in "#." for r in ice for c in r):
        return "Every character is # or ."
    return None
