def holds(ops: list[str], args: list[list[int]]) -> list[int]:
    # Obviously correct: one plain list per book, updated exactly as described.
    lines: dict[int, list[int]] = {}
    reports = []
    for op, a in zip(ops, args):
        line = lines.setdefault(a[0], [])
        if op == "hold":
            line.append(a[1])
        elif op == "cancel":
            if a[1] in line:
                line.remove(a[1])
        else:
            reports.append(line.pop(0) if line else -1)
    return reports
