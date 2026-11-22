def row_ledger(stock: list[int], ops: list[str], args: list[list[int]]) -> list[int]:
    n = len(stock)
    # Two Fenwick trees for range add / range sum: prefix(i) = b1(i) * i - b2(i).
    b1 = [0] * (n + 1)
    b2 = [0] * (n + 1)

    def add(tree: list[int], i: int, v: int) -> None:
        while i <= n:
            tree[i] += v
            i += i & -i

    def query(tree: list[int], i: int) -> int:
        s = 0
        while i > 0:
            s += tree[i]
            i -= i & -i
        return s

    def range_add(a: int, b: int, v: int) -> None:  # shelves a..b, 1-based
        add(b1, a, v)
        add(b1, b + 1, -v)
        add(b2, a, v * (a - 1))
        add(b2, b + 1, -v * b)

    def prefix(i: int) -> int:
        return query(b1, i) * i - query(b2, i)

    for i, s in enumerate(stock):
        range_add(i + 1, i + 1, s)
    out = []
    for op, a in zip(ops, args):
        if op == "add":
            range_add(a[0] + 1, a[1] + 1, a[2])
        else:
            out.append(prefix(a[1] + 1) - prefix(a[0]))
    return out
