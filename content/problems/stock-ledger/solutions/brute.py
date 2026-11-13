def stock_ledger(stock: list[int], ops: list[str], args: list[list[int]]) -> list[int]:
    # Obviously correct: keep the shelves and sum each range.
    shelves = list(stock)
    out = []
    for op, a in zip(ops, args):
        if op == "set":
            shelves[a[0]] = a[1]
        else:
            out.append(sum(shelves[a[0] : a[1] + 1]))
    return out
