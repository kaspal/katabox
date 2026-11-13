def stock_ledger(stock: list[int], ops: list[str], args: list[list[int]]) -> list[int]:
    n = len(stock)
    tree = [0] * (n + 1)

    def add(i: int, delta: int) -> None:
        i += 1
        while i <= n:
            tree[i] += delta
            i += i & -i

    def prefix(i: int) -> int:  # total of positions 0 .. i - 1
        total = 0
        while i > 0:
            total += tree[i]
            i -= i & -i
        return total

    current = list(stock)
    for i, v in enumerate(stock):
        add(i, v)
    out = []
    for op, a in zip(ops, args):
        if op == "set":
            i, count = a
            add(i, count - current[i])
            current[i] = count
        else:
            out.append(prefix(a[1] + 1) - prefix(a[0]))
    return out
