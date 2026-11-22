"""checkout-merge--f1 constraints (lines arrive as arrays: ADR 0025)."""


def validate(inp):
    lines = inp["lines"]
    if not isinstance(lines, list) or not 1 <= len(lines) <= 10**4:
        return "1 ≤ lines.length ≤ 10⁴"
    total = 0
    for line in lines:
        if not isinstance(line, list) or not line:
            return "every line has at least one customer"
        if not all(type(v) is int and 1 <= v <= 10**6 for v in line):
            return "1 ≤ ticket number ≤ 10⁶"
        if any(line[i] > line[i + 1] for i in range(len(line) - 1)):
            return "each line is in non-decreasing order"
        total += len(line)
    if total > 10**5:
        return "total number of customers ≤ 10⁵"
    return None
