from collections import OrderedDict


def recent_pages(capacity: int, ops: list[str], args: list[list[int]]) -> list[int]:
    cache: OrderedDict[int, int] = OrderedDict()
    out = []
    for op, a in zip(ops, args):
        page = a[0]
        if op == "get":
            if page in cache:
                cache.move_to_end(page)
                out.append(cache[page])
            else:
                out.append(-1)
        else:
            cache[page] = a[1]
            cache.move_to_end(page)
            if len(cache) > capacity:
                cache.popitem(last=False)
    return out
