def validate(inp):
    limit, window, ops, args = inp["limit"], inp["window"], inp["ops"], inp["args"]
    if type(limit) is not int or not 1 <= limit <= 1000 or type(window) is not int or not 1 <= window <= 10**9:
        return "1 ≤ limit ≤ 1000, 1 ≤ window ≤ 10⁹"
    if not isinstance(ops, list) or not isinstance(args, list) or not 1 <= len(ops) == len(args) <= 10**5:
        return "1 ≤ ops.length = args.length ≤ 10⁵"
    last = 0
    for op, a in zip(ops, args):
        if op not in ("request", "count"):
            return "ops are request or count"
        if not isinstance(a, list) or len(a) != 2 or not all(type(v) is int for v in a):
            return "args are [user, time]"
        if not (1 <= a[0] <= 10**9 and 0 <= a[1] <= 10**9) or a[1] < last:
            return "1 ≤ user ≤ 10⁹, 0 ≤ time ≤ 10⁹, times never decrease"
        last = a[1]
    return None
