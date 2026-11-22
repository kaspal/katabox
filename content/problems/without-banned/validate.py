"""without-banned constraints."""


def validate(inp):
    items, banned = inp["items"], inp["banned"]
    for name, a, lo in (("items", items, 1), ("banned", banned, 0)):
        if not isinstance(a, list) or not all(type(x) is int and 1 <= x <= 10**9 for x in a):
            return f"{name} holds integers 1 ≤ x ≤ 10⁹"
    if not 1 <= len(items) <= 10**5 or len(banned) > 10**5:
        return "1 ≤ items.length ≤ 10⁵, 0 ≤ banned.length ≤ 10⁵"
    return None
