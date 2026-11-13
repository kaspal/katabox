def recent_pages(capacity: int, ops: list[str], args: list[list[int]]) -> list[int]:
    # Obviously correct: remember when each page was last used; evict the minimum.
    value, used = {}, {}
    out = []
    for now, (op, a) in enumerate(zip(ops, args)):
        page = a[0]
        if op == "get":
            if page in value:
                used[page] = now
                out.append(value[page])
            else:
                out.append(-1)
        else:
            value[page] = a[1]
            used[page] = now
            if len(value) > capacity:
                oldest = min(value, key=lambda p: used[p])
                del value[oldest], used[oldest]
    return out
