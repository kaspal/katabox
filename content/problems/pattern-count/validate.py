import re


def validate(inp):
    t, p = inp["text"], inp["pattern"]
    if not isinstance(t, str) or not isinstance(p, str):
        return "text and pattern are strings"
    if not (1 <= len(t) <= 2 * 10**5 and 1 <= len(p) <= 10**5):
        return "1 ≤ text.length ≤ 2·10⁵, 1 ≤ pattern.length ≤ 10⁵"
    if not re.fullmatch(r"[a-z]+", t) or not re.fullmatch(r"[a-z]+", p):
        return "lowercase English letters only"
    return None
