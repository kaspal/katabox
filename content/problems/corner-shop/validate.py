ARITY = {"stock": 2, "sell": 2, "count": 1}


def validate(inp):
    ops, args = inp["ops"], inp["args"]
    if not isinstance(ops, list) or not isinstance(args, list):
        return "ops and args are arrays"
    if not 1 <= len(ops) <= 10**4 or len(args) != len(ops):
        return "1 ≤ ops.length ≤ 10⁴ and args.length = ops.length"
    for op, a in zip(ops, args):
        if op not in ARITY:
            return "ops[i] is \"stock\", \"sell\" or \"count\""
        if not isinstance(a, list) or len(a) != ARITY[op] or not all(type(x) is int for x in a):
            return "stock and sell take [item, qty]; count takes [item]"
        if not 1 <= a[0] <= 10**9:
            return "1 ≤ item ≤ 10⁹"
        if len(a) == 2 and not 1 <= a[1] <= 10**4:
            return "1 ≤ qty ≤ 10⁴"
    return None
