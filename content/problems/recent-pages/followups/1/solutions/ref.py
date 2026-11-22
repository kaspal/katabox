def favourite_pages(capacity: int, ops: list[str], args: list[list[int]]) -> list[int]:
    value: dict[int, int] = {}
    freq: dict[int, int] = {}
    # buckets[f]: the pages used f times, oldest use first (dicts keep insertion order).
    buckets: dict[int, dict[int, None]] = {}
    least = 0
    out = []

    def touch(page: int) -> None:
        nonlocal least
        f = freq[page]
        del buckets[f][page]
        if not buckets[f]:
            del buckets[f]
            if least == f:
                least = f + 1
        freq[page] = f + 1
        buckets.setdefault(f + 1, {})[page] = None

    for op, a in zip(ops, args):
        if op == "get":
            page = a[0]
            if page in value:
                touch(page)
                out.append(value[page])
            else:
                out.append(-1)
        else:
            page, v = a
            if page in value:
                value[page] = v
                touch(page)
                continue
            if len(value) == capacity:
                old = next(iter(buckets[least]))
                del buckets[least][old]
                if not buckets[least]:
                    del buckets[least]
                del value[old], freq[old]
            value[page] = v
            freq[page] = 1
            buckets.setdefault(1, {})[page] = None
            least = 1
    return out
