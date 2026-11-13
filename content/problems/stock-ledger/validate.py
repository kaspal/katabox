def validate(inp):
    stock, ops, args = inp["stock"], inp["ops"], inp["args"]
    if not isinstance(stock, list) or not all(type(v) is int for v in stock):
        return "stock is a list of integers"
    if not isinstance(ops, list) or not isinstance(args, list) or len(ops) != len(args):
        return "ops.length = args.length"
    n = len(stock)
    if not 1 <= n <= 10**5:
        return "1 ≤ stock.length ≤ 10⁵"
    if not all(0 <= v <= 10**4 for v in stock):
        return "0 ≤ stock[i], count ≤ 10⁴"
    if not 1 <= len(ops) <= 10**5:
        return "1 ≤ ops.length ≤ 10⁵, ops.length = args.length"
    for op, a in zip(ops, args):
        if not isinstance(a, list) or len(a) != 2 or not all(type(v) is int for v in a):
            return "each args entry has two integers"
        if op == "set":
            if not (0 <= a[0] < n and 0 <= a[1] <= 10**4):
                return "\"set\" has [i, count] with 0 ≤ i < stock.length"
        elif op == "total":
            if not 0 <= a[0] <= a[1] < n:
                return "\"total\" has [a, b] with 0 ≤ a ≤ b < stock.length"
        else:
            return "ops are \"set\" or \"total\""
    return None
