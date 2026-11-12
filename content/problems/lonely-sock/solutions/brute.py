def lone_sock(ids: list[int]) -> int:
    # Obviously correct: the value that appears exactly once.
    for x in ids:
        if ids.count(x) == 1:
            return x
    return -1
