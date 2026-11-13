from collections import deque


def throttle(limit: int, window: int, ops: list[str], args: list[list[int]]) -> list[int]:
    logs: dict[int, deque] = {}
    out = []
    for op, (user, time) in zip(ops, args):
        log = logs.setdefault(user, deque())
        while log and log[0] <= time - window:
            log.popleft()
        if op == "count":
            out.append(len(log))
        elif len(log) < limit:
            log.append(time)
            out.append(1)
        else:
            out.append(0)
    return out
