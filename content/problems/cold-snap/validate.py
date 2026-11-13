def validate(inp):
    temps, ops, args = inp["temps"], inp["ops"], inp["args"]
    if not isinstance(temps, list) or not all(type(v) is int for v in temps):
        return "temps is a list of integers"
    if not isinstance(ops, list) or not isinstance(args, list) or len(ops) != len(args):
        return "ops.length = args.length"
    n = len(temps)
    if not 1 <= n <= 10**5:
        return "1 ≤ temps.length ≤ 10⁵"
    if not all(-1000 <= v <= 1000 for v in temps):
        return "-1000 ≤ temps[i] ≤ 1000"
    if not 1 <= len(ops) <= 10**5:
        return "1 ≤ ops.length ≤ 10⁵, ops.length = args.length"
    for op, a in zip(ops, args):
        if not isinstance(a, list) or not all(type(v) is int for v in a):
            return "each args entry is a list of integers"
        if op == "shift":
            if len(a) != 3 or not (0 <= a[0] <= a[1] < n and -50 <= a[2] <= 50):
                return "\"shift\" has [a, b, delta] with 0 ≤ a ≤ b < temps.length and -50 ≤ delta ≤ 50"
        elif op == "coldest":
            if len(a) != 2 or not 0 <= a[0] <= a[1] < n:
                return "\"coldest\" has [a, b] with 0 ≤ a ≤ b < temps.length"
        else:
            return "ops are \"shift\" or \"coldest\""
    return None
