def is_list(v, lo, hi):
    return isinstance(v, list) and all(type(x) is int and lo <= x <= hi for x in v)


def validate(inp):
    a, b = inp["first"], inp["second"]
    if not is_list(a, 1, 10**6) or not is_list(b, 1, 10**6):
        return "1 ≤ ticket number ≤ 10⁶"
    if len(a) > 5 * 10**4 or len(b) > 5 * 10**4:
        return "0 ≤ length of each queue ≤ 5·10⁴"
    if any(q[i] > q[i + 1] for q in (a, b) for i in range(len(q) - 1)):
        return "Each queue is in non-decreasing order."
    return None
