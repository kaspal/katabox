def cold_snaps(temps: list[int], ops: list[str], args: list[list[int]]) -> list[int]:
    # Correct but O(n·q): every operation walks its range.
    t = list(temps)
    out = []
    for op, a in zip(ops, args):
        if op == "shift":
            for i in range(a[0], a[1] + 1):
                t[i] += a[2]
        else:
            low = t[a[0]]
            for i in range(a[0], a[1] + 1):
                if t[i] < low:
                    low = t[i]
            out.append(low)
    return out
