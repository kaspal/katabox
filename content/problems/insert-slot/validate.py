def validate(inp):
    s, f = inp["slots"], inp["fresh"]
    if not isinstance(s, list) or len(s) > 10**4:
        return "0 ≤ slots.length ≤ 10⁴"
    for x in s + [f]:
        if not isinstance(x, list) or len(x) != 2 or not all(type(v) is int for v in x) or not 0 <= x[0] <= x[1] <= 10**6:
            return "0 ≤ start ≤ end ≤ 10⁶"
    for a, b in zip(s, s[1:]):
        if b[0] <= a[1]:
            return "slots are sorted and each starts after the previous one ends"
    return None
