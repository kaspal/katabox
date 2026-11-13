def cold_snaps(temps: list[int], ops: list[str], args: list[list[int]]) -> list[int]:
    n = len(temps)
    size = 1
    while size < n:
        size *= 2
    inf = float("inf")
    mn = [inf] * (2 * size)
    lazy = [0] * (2 * size)
    mn[size : size + n] = temps
    for v in range(size - 1, 0, -1):
        mn[v] = min(mn[2 * v], mn[2 * v + 1])

    def push(v):
        if lazy[v]:
            for c in (2 * v, 2 * v + 1):
                mn[c] += lazy[v]
                lazy[c] += lazy[v]
            lazy[v] = 0

    def shift(v, lo, hi, a, b, d):
        if b < lo or hi < a:
            return
        if a <= lo and hi <= b:
            mn[v] += d
            lazy[v] += d
            return
        push(v)
        mid = (lo + hi) // 2
        shift(2 * v, lo, mid, a, b, d)
        shift(2 * v + 1, mid + 1, hi, a, b, d)
        mn[v] = min(mn[2 * v], mn[2 * v + 1])

    def coldest(v, lo, hi, a, b):
        if b < lo or hi < a:
            return inf
        if a <= lo and hi <= b:
            return mn[v]
        push(v)
        mid = (lo + hi) // 2
        return min(coldest(2 * v, lo, mid, a, b), coldest(2 * v + 1, mid + 1, hi, a, b))

    out = []
    for op, a in zip(ops, args):
        if op == "shift":
            shift(1, 0, size - 1, a[0], a[1], a[2])
        else:
            out.append(coldest(1, 0, size - 1, a[0], a[1]))
    return out
