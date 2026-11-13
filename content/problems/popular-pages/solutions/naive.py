def popular_pages(capacity: int, ops: list[str], args: list[list[int]]) -> list[int]:
    # Correct but O(capacity) per operation: a plain list of [page, value, count, last use].
    cache = []
    out = []
    for now, (op, a) in enumerate(zip(ops, args)):
        page = a[0]
        entry = next((e for e in cache if e[0] == page), None)
        if entry is not None:
            entry[2] += 1
            entry[3] = now
            if op == "get":
                out.append(entry[1])
            else:
                entry[1] = a[1]
        elif op == "get":
            out.append(-1)
        else:
            if len(cache) == capacity:
                cache.remove(min(cache, key=lambda e: (e[2], e[3])))
            cache.append([page, a[1], 1, now])
    return out
