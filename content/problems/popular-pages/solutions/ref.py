from collections import OrderedDict


def popular_pages(capacity: int, ops: list[str], args: list[list[int]]) -> list[int]:
    value: dict[int, int] = {}
    count: dict[int, int] = {}
    bucket: dict[int, OrderedDict] = {}
    min_count = 0
    out = []

    def use(page):
        nonlocal min_count
        c = count[page]
        del bucket[c][page]
        if not bucket[c]:
            del bucket[c]
            if min_count == c:
                min_count = c + 1
        count[page] = c + 1
        bucket.setdefault(c + 1, OrderedDict())[page] = None

    for op, a in zip(ops, args):
        page = a[0]
        if op == "get":
            if page in value:
                use(page)
                out.append(value[page])
            else:
                out.append(-1)
        elif page in value:
            value[page] = a[1]
            use(page)
        else:
            if len(value) == capacity:
                old, _ = bucket[min_count].popitem(last=False)
                if not bucket[min_count]:
                    del bucket[min_count]
                del value[old], count[old]
            value[page], count[page] = a[1], 1
            bucket.setdefault(1, OrderedDict())[page] = None
            min_count = 1
    return out
