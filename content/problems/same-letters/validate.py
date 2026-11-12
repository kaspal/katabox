import string


def validate(inp):
    a, b = inp["a"], inp["b"]
    if not isinstance(a, str) or not isinstance(b, str):
        return "a and b are strings"
    if not 1 <= len(a) <= 10**5 or not 1 <= len(b) <= 10**5:
        return "1 ≤ a.length, b.length ≤ 10⁵"
    letters = set(string.ascii_lowercase)
    if not set(a) <= letters or not set(b) <= letters:
        return "a and b contain only lowercase English letters."
    return None
