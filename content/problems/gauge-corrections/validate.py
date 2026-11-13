def validate(inp):
    readings, ops, args = inp["readings"], inp["ops"], inp["args"]
    if not isinstance(readings, list) or not all(type(v) is int for v in readings):
        return "readings is a list of integers"
    if not isinstance(ops, list) or not isinstance(args, list) or len(ops) != len(args):
        return "ops.length = args.length"
    n = len(readings)
    if not 1 <= n <= 10**5:
        return "1 ≤ readings.length ≤ 10⁵"
    if not all(-(10**4) <= v <= 10**4 for v in readings):
        return "-10⁴ ≤ readings[i] ≤ 10⁴"
    if not 1 <= len(ops) <= 10**5:
        return "1 ≤ ops.length ≤ 10⁵, ops.length = args.length"
    for op, a in zip(ops, args):
        if not isinstance(a, list) or not all(type(v) is int for v in a):
            return "each args entry is a list of integers"
        if op == "add":
            if len(a) != 3 or not (0 <= a[0] <= a[1] < n and -100 <= a[2] <= 100):
                return "\"add\" has [a, b, delta] with 0 ≤ a ≤ b < readings.length and -100 ≤ delta ≤ 100"
        elif op == "read":
            if len(a) != 1 or not 0 <= a[0] < n:
                return "\"read\" has [i] with 0 ≤ i < readings.length"
        else:
            return "ops are \"add\" or \"read\""
    return None
