def popular_pages(capacity: int, ops: list[str], args: list[list[int]]) -> list[int]:
    # Obviously correct: remember (count, last use) per page; evict the minimum.
    value, count, used = {}, {}, {}
    out = []
    for now, (op, a) in enumerate(zip(ops, args)):
        page = a[0]
        if page in value:
            count[page] += 1
            used[page] = now
            if op == "get":
                out.append(value[page])
            else:
                value[page] = a[1]
        elif op == "get":
            out.append(-1)
        else:
            if len(value) == capacity:
                old = min(value, key=lambda p: (count[p], used[p]))
                del value[old], count[old], used[old]
            value[page], count[page], used[page] = a[1], 1, now
    return out
