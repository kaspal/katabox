"""stock-ledger--f1 constraints."""


def validate(inp):
    stock, ops, args = inp["stock"], inp["ops"], inp["args"]
    if not isinstance(stock, list) or not 1 <= len(stock) <= 10**5:
        return "1 ≤ stock.length ≤ 10⁵"
    if not all(type(x) is int and 0 <= x <= 10**4 for x in stock):
        return "0 ≤ stock[i] ≤ 10⁴"
    if not isinstance(ops, list) or not isinstance(args, list) or len(ops) != len(args) or not 1 <= len(ops) <= 10**5:
        return "1 ≤ ops.length ≤ 10⁵, ops.length = args.length"
    n = len(stock)
    for op, a in zip(ops, args):
        if not isinstance(a, list) or not all(type(x) is int for x in a):
            return "args are integers"
        if op == "add":
            ok = len(a) == 3 and 0 <= a[0] <= a[1] < n and 0 <= a[2] <= 10**4
        elif op == "total":
            ok = len(a) == 2 and 0 <= a[0] <= a[1] < n
        else:
            return "each op is add or total"
        if not ok:
            return "add has [a, b, count] with 0 ≤ count ≤ 10⁴; total has [a, b]; 0 ≤ a ≤ b < stock.length"
    return None
