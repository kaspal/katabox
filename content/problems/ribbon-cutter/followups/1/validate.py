"""ribbon-cutter--f1 constraints."""


def validate(inp):
    r, o = inp["ribbons"], inp["orders"]
    for name, a in (("ribbons", r), ("orders", o)):
        if not isinstance(a, list) or not all(type(x) is int for x in a):
            return f"{name} is an array of integers"
    if not 1 <= len(r) <= 10**5 or not 1 <= len(o) <= 10**5:
        return "1 ≤ ribbons.length, orders.length ≤ 10⁵"
    if not all(1 <= x <= 10**5 for x in r):
        return "1 ≤ ribbons[i] ≤ 10⁵"
    if not all(1 <= x <= 10**10 for x in o):
        return "1 ≤ orders[k] ≤ 10¹⁰"
    return None
