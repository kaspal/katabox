"""club-merges--f2 constraints."""


def validate(inp):
    n, ops = inp["n"], inp["ops"]
    if type(n) is not int or not 1 <= n <= 10**5:
        return "1 ≤ n ≤ 10⁵"
    if not isinstance(ops, list) or not 1 <= len(ops) <= 10**5:
        return "1 ≤ ops.length ≤ 10⁵"
    for op in ops:
        ok = isinstance(op, list) and all(type(x) is int for x in op) and (
            op == [0] or (len(op) == 3 and op[0] == 1 and 0 <= op[1] < n and 0 <= op[2] < n))
        if not ok:
            return "each op is [0] (undo) or [1, a, b] (merge) with 0 ≤ a, b < n"
    return None
