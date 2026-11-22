"""bracket-checker constraints."""


def validate(inp):
    s = inp["code"]
    if not isinstance(s, str) or not 1 <= len(s) <= 10**5:
        return "1 ≤ code.length ≤ 10⁵"
    if any(c not in "()[]{}" for c in s):
        return "code contains only ()[]{}"
    return None
