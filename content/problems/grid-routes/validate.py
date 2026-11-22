"""grid-routes constraints."""


def validate(inp):
    f = inp["floor"]
    if not isinstance(f, list) or not 1 <= len(f) <= 100:
        return "1 ≤ rows ≤ 100"
    if not all(isinstance(r, str) for r in f) or not 1 <= len(f[0]) <= 100:
        return "1 ≤ columns ≤ 100"
    if any(len(r) != len(f[0]) or set(r) - {".", "#"} for r in f):
        return "every row has the same length and only '.' or '#'"
    return None
