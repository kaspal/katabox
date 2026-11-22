"""nested-brackets--f2 constraints."""


def validate(inp):
    t = inp["text"]
    if not isinstance(t, str) or not 1 <= len(t) <= 10**5:
        return "1 ≤ text.length ≤ 10⁵"
    if any(c not in "()[]{}" for c in t):
        return "text contains only ()[]{}"
    return None
