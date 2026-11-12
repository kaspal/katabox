def holds(ops: list[str], args: list[list[int]]) -> list[int]:
    # Correct but O(ops²): a plain list per book; cancelling searches the whole line.
    lines = {}
    reports = []
    for op, a in zip(ops, args):
        line = lines.setdefault(a[0], [])
        if op == "hold":
            line.append(a[1])
        elif op == "cancel":
            for k in range(len(line)):
                if line[k] == a[1]:
                    del line[k]
                    break
        else:
            reports.append(line.pop(0) if line else -1)
    return reports
