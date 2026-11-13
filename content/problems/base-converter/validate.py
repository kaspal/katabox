DIGITS = "0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZ"


def validate(inp):
    s, a, b = inp["number"], inp["source"], inp["target"]
    if type(a) is not int or type(b) is not int or not (2 <= a <= 36 and 2 <= b <= 36):
        return "2 ≤ source, target ≤ 36"
    if not isinstance(s, str) or not s or any(c not in DIGITS[:a] for c in s):
        return "number uses only digits valid in base source"
    if len(s) > 1 and s[0] == "0":
        return "no leading zeros"
    if int(s, a) > 10**15:
        return "the value is at most 10¹⁵"
    return None
