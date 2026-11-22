"""nested-brackets--f1 constraints."""


def validate(inp):
    t = inp["text"]
    if not isinstance(t, str) or not 1 <= len(t) <= 10**5:
        return "1 ≤ text.length ≤ 10⁵"
    if any(c not in "()" and not "a" <= c <= "z" for c in t):
        return "text contains only lowercase English letters, ( and )"
    return None
