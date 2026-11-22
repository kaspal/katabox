"""checkout-merge--f1 constraints."""


def validate(inp):
    lines = inp["lines"]
    if not isinstance(lines, list) or len(lines) > 10**4:
        return "0 ≤ lines.length ≤ 10⁴"
    total = 0
    for l in lines:
        if not isinstance(l, list) or not all(type(x) is int and 1 <= x <= 10**6 for x in l):
            return "1 ≤ ticket number ≤ 10⁶"
        if l != sorted(l):
            return "Each line is in non-decreasing order."
        total += len(l)
    if total > 10**5:
        return "0 ≤ total number of customers ≤ 10⁵"
    return None
