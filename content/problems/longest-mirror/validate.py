import re


def validate(inp):
    t = inp["text"]
    if not isinstance(t, str) or not 1 <= len(t) <= 3000:
        return "1 ≤ text.length ≤ 3000"
    if not re.fullmatch(r"[a-z]+", t):
        return "lowercase English letters only"
    return None
