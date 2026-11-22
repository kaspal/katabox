"""inclusive-total constraints."""

def _ints(name, a, lo, hi, nmin, nmax):
    if not isinstance(a, list) or not all(type(x) is int for x in a):
        return f"{name} is an array of integers"
    if not nmin <= len(a) <= nmax:
        return f"{nmin} ≤ {name}.length ≤ {nmax}"
    if not all(lo <= x <= hi for x in a):
        return f"{lo} ≤ {name}[i] ≤ {hi}"
    return None


def validate(inp):
    err = _ints("nums", inp["nums"], -1000, 1000, 1, 10**5)
    if err:
        return err
    a, b, n = inp["a"], inp["b"], len(inp["nums"])
    if type(a) is not int or type(b) is not int or not 0 <= a <= b < n:
        return "0 ≤ a ≤ b < nums.length"
    return None
