import re


def validate(inp):
    s, t = inp["stalls"], inp["need"]
    if not isinstance(s, str) or not isinstance(t, str):
        return "stalls and need are strings"
    if not (1 <= len(s) <= 10**5 and 1 <= len(t) <= 10**5):
        return "1 ≤ stalls.length, need.length ≤ 10⁵"
    if not re.fullmatch(r"[a-z]+", s) or not re.fullmatch(r"[a-z]+", t):
        return "lowercase English letters only"
    return None
