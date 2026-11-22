def row_ledger(stock: list[int], ops: list[str], args: list[list[int]]) -> list[int]:
    # Obviously correct: update and add up shelf by shelf.
    shelves = list(stock)
    out = []
    for op, a in zip(ops, args):
        if op == "add":
            for i in range(a[0], a[1] + 1):
                shelves[i] += a[2]
        else:
            out.append(sum(shelves[a[0] : a[1] + 1]))
    return out
