def validate(inp):
    cap, ops, args = inp["capacity"], inp["ops"], inp["args"]
    if type(cap) is not int or not 1 <= cap <= 10**5:
        return "1 ≤ capacity ≤ 10⁵"
    if not isinstance(ops, list) or not isinstance(args, list) or not 1 <= len(ops) == len(args) <= 2 * 10**5:
        return "1 ≤ ops.length = args.length ≤ 2·10⁵"
    for op, a in zip(ops, args):
        if not isinstance(a, list) or not all(type(v) is int for v in a):
            return "args are lists of integers"
        if op == "get" and len(a) == 1 and 1 <= a[0] <= 10**9:
            continue
        if op == "put" and len(a) == 2 and 1 <= a[0] <= 10**9 and 0 <= a[1] <= 10**9:
            continue
        return "get [page] or put [page, value], 1 ≤ page ≤ 10⁹, 0 ≤ value ≤ 10⁹"
    return None
