import string


def validate(inp):
    t = inp["text"]
    if not isinstance(t, str) or not 1 <= len(t) <= 1500:
        return "1 ≤ text.length ≤ 1500"
    if not all(c in string.ascii_lowercase for c in t):
        return "text has lowercase English letters only."
    return None
