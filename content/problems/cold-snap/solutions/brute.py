def cold_snaps(temps: list[int], ops: list[str], args: list[list[int]]) -> list[int]:
    # Obviously correct: keep every forecast and scan each range.
    t = list(temps)
    out = []
    for op, a in zip(ops, args):
        if op == "shift":
            for i in range(a[0], a[1] + 1):
                t[i] += a[2]
        else:
            out.append(min(t[a[0] : a[1] + 1]))
    return out
