"""longest-streak constraints."""

def _ints(name, a, lo, hi, nmin, nmax):
    if not isinstance(a, list) or not all(type(x) is int for x in a):
        return f"{name} is an array of integers"
    if not nmin <= len(a) <= nmax:
        return f"{nmin} ≤ {name}.length ≤ {nmax}"
    if not all(lo <= x <= hi for x in a):
        return f"{lo} ≤ {name}[i] ≤ {hi}"
    return None


def validate(inp):
    return _ints("values", inp["values"], -(10**9), 10**9, 1, 10**5)
