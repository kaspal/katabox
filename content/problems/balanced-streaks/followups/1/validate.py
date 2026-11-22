"""balanced-streaks--f1 constraints."""


def validate(inp):
    s = inp["results"]
    if not isinstance(s, str) or not 1 <= len(s) <= 10**5:
        return "1 ≤ results.length ≤ 10⁵"
    if any(c not in "WLD" for c in s):
        return "results contains only W, L and D"
    return None
