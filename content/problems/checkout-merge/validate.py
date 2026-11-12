def validate(inp):
    lines = inp["lines"]
    if not isinstance(lines, list) or not all(isinstance(l, list) and all(type(v) is int for v in l) for l in lines):
        return "lines is a list of lists of integers"
    if len(lines) > 10**4:
        return "0 ≤ lines.length ≤ 10⁴"
    if sum(len(l) for l in lines) > 10**5:
        return "0 ≤ total number of customers ≤ 10⁵"
    if not all(1 <= v <= 10**6 for l in lines for v in l):
        return "1 ≤ ticket number ≤ 10⁶"
    if any(l[i] > l[i + 1] for l in lines for i in range(len(l) - 1)):
        return "Each line is in non-decreasing order."
    return None
