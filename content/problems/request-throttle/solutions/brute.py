def throttle(limit: int, window: int, ops: list[str], args: list[list[int]]) -> list[int]:
    # Obviously correct: keep every allowed request and count the window from scratch.
    allowed = []
    out = []
    for op, (user, time) in zip(ops, args):
        inside = sum(1 for u, t in allowed if u == user and time - window < t <= time)
        if op == "count":
            out.append(inside)
        elif inside < limit:
            allowed.append((user, time))
            out.append(1)
        else:
            out.append(0)
    return out
