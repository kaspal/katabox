ARITY = {"write": 2, "read": 1, "undo": 0, "redo": 0}


def validate(inp):
    ops, args = inp["ops"], inp["args"]
    if not isinstance(ops, list) or not isinstance(args, list):
        return "ops and args are arrays"
    if not 1 <= len(ops) <= 10**5 or len(args) != len(ops):
        return "1 ≤ ops.length ≤ 10⁵ and args.length = ops.length"
    for op, a in zip(ops, args):
        if op not in ARITY:
            return "ops[i] is write, read, undo or redo"
        if not isinstance(a, list) or len(a) != ARITY[op] or not all(type(x) is int for x in a):
            return "write takes [key, value], read [key], undo and redo []"
        if a and not 1 <= a[0] <= 10**9:
            return "1 ≤ key ≤ 10⁹"
        if len(a) == 2 and not 0 <= a[1] <= 10**9:
            return "0 ≤ value ≤ 10⁹"
    return None
