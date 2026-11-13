def gauge_readings(readings: list[int], ops: list[str], args: list[list[int]]) -> list[int]:
    # Correct but O(n·q): every "add" walks its range.
    levels = list(readings)
    out = []
    for op, a in zip(ops, args):
        if op == "add":
            for i in range(a[0], a[1] + 1):
                levels[i] += a[2]
        else:
            out.append(levels[a[0]])
    return out
