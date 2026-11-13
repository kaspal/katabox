def stock_ledger(stock: list[int], ops: list[str], args: list[list[int]]) -> list[int]:
    # Correct but O(n·q): every "total" walks its shelves.
    shelves = list(stock)
    out = []
    for op, a in zip(ops, args):
        if op == "set":
            shelves[a[0]] = a[1]
        else:
            total = 0
            for i in range(a[0], a[1] + 1):
                total += shelves[i]
            out.append(total)
    return out
