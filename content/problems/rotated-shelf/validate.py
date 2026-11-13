def validate(inp):
    s, label = inp["shelf"], inp["label"]
    if not isinstance(s, list) or not 1 <= len(s) <= 10**5 or type(label) is not int:
        return "1 ≤ shelf.length ≤ 10⁵; label is an integer"
    if not all(type(v) is int and 0 <= v <= 10**9 for v in s) or not 0 <= label <= 10**9:
        return "0 ≤ shelf[i], label ≤ 10⁹"
    breaks = sum(1 for i in range(len(s) - 1) if s[i] >= s[i + 1])
    if breaks > 1 or (breaks == 1 and s[-1] >= s[0]) or len(set(s)) != len(s):
        return "a sorted list of different numbers with a block moved to the end"
    return None
