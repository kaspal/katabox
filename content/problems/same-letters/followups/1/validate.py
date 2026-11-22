"""same-letters--f1 constraints."""


def validate(inp):
    a, b, p, l = inp["a"], inp["b"], inp["positions"], inp["letters"]
    if not isinstance(a, str) or not isinstance(b, str) or not isinstance(l, str) or not isinstance(p, list):
        return "a, b and letters are strings; positions is an array"
    if not 1 <= len(a) <= 10**5 or not 1 <= len(b) <= 10**5:
        return "1 ≤ a.length, b.length ≤ 10⁵"
    if not 1 <= len(p) <= 10**5 or len(l) != len(p):
        return "1 ≤ positions.length = letters.length ≤ 10⁵"
    if not all(type(x) is int and 0 <= x < len(b) for x in p):
        return "0 ≤ positions[i] < b.length"
    if not all("a" <= c <= "z" for c in a + b + l):
        return "only lowercase English letters"
    return None
