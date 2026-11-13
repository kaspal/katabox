def gauge_readings(readings: list[int], ops: list[str], args: list[list[int]]) -> list[int]:
    n = len(readings)
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

    out = []
    for op, a in zip(ops, args):
        if op == "add":
            add(a[0], a[2])
            if a[1] + 1 < n:
                add(a[1] + 1, -a[2])
        else:
            out.append(readings[a[0]] + prefix(a[0] + 1))
    return out
