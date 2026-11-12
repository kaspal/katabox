def lone_sock(ids: list[int]) -> int:
    acc = 0
    for x in ids:
        acc ^= x
    return acc
