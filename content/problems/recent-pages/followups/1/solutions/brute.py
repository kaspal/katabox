def favourite_pages(capacity: int, ops: list[str], args: list[list[int]]) -> list[int]:
    # Obviously correct: keep (page, value, uses, last use) and scan for the one to evict.
    cache: dict[int, list[int]] = {}
    out = []
    for tick, (op, a) in enumerate(zip(ops, args)):
        if op == "get":
            if a[0] in cache:
                e = cache[a[0]]
                e[1] += 1
                e[2] = tick
                out.append(e[0])
            else:
                out.append(-1)
            continue
        page, v = a
        if page in cache:
            e = cache[page]
            e[0], e[1], e[2] = v, e[1] + 1, tick
            continue
        if len(cache) == capacity:
            victim = min(cache, key=lambda p: (cache[p][1], cache[p][2]))
            del cache[victim]
        cache[page] = [v, 1, tick]
    return out
