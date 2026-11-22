"""recent-pages--f1 constraints."""


def validate(inp):
    cap, ops, args = inp["capacity"], inp["ops"], inp["args"]
    if type(cap) is not int or not 1 <= cap <= 10**5:
        return "1 ≤ capacity ≤ 10⁵"
    if not isinstance(ops, list) or not isinstance(args, list) or len(ops) != len(args):
        return "args.length = ops.length"
    if not 1 <= len(ops) <= 2 * 10**5:
        return "1 ≤ ops.length ≤ 2·10⁵"
    for op, a in zip(ops, args):
        if op == "get":
            ok = isinstance(a, list) and len(a) == 1 and type(a[0]) is int and 1 <= a[0] <= 10**9
        elif op == "put":
            ok = (isinstance(a, list) and len(a) == 2 and all(type(x) is int for x in a)
                  and 1 <= a[0] <= 10**9 and 0 <= a[1] <= 10**9)
        else:
            return "each op is get or put"
        if not ok:
            return "get takes [page], put takes [page, value]; 1 ≤ page ≤ 10⁹, 0 ≤ value ≤ 10⁹"
    return None
