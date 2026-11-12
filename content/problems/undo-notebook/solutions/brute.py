def notebook(ops: list[str], args: list[list[int]]) -> list[int]:
    # Obviously correct: keep the list of writes in effect and the undone ones; a read
    # replays the writes in effect from the start.
    done, undone, reports = [], [], []
    for op, a in zip(ops, args):
        if op == "write":
            done.append((a[0], a[1]))
            undone = []
        elif op == "read":
            value = -1
            for k, v in done:
                if k == a[0]:
                    value = v
            reports.append(value)
        elif op == "undo":
            if done:
                undone.append(done.pop())
        elif undone:
            done.append(undone.pop())
    return reports
