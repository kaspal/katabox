def notebook(ops: list[str], args: list[list[int]]) -> list[int]:
    # Correct but O(ops²): a read searches the writes in effect from the newest.
    done, undone, reports = [], [], []
    for op, a in zip(ops, args):
        if op == "write":
            done.append((a[0], a[1]))
            undone = []
        elif op == "read":
            value = -1
            for i in range(len(done) - 1, -1, -1):
                if done[i][0] == a[0]:
                    value = done[i][1]
                    break
            reports.append(value)
        elif op == "undo":
            if done:
                undone.append(done.pop())
        elif undone:
            done.append(undone.pop())
    return reports
