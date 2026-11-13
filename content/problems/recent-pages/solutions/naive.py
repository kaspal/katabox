def recent_pages(capacity: int, ops: list[str], args: list[list[int]]) -> list[int]:
    # Correct but O(capacity) per operation: a list of [page, value], oldest first.
    cache = []
    out = []
    for op, a in zip(ops, args):
        page = a[0]
        at = next((i for i, e in enumerate(cache) if e[0] == page), -1)
        if op == "get":
            if at < 0:
                out.append(-1)
            else:
                entry = cache.pop(at)
                cache.append(entry)
                out.append(entry[1])
        else:
            if at >= 0:
                cache.pop(at)
            cache.append([page, a[1]])
            if len(cache) > capacity:
                cache.pop(0)
    return out
