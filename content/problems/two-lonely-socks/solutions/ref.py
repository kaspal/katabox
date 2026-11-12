def two_lonely(ids: list[int]) -> list[int]:
    t = 0
    for x in ids:
        t ^= x
    bit = t & -t
    a = 0
    for x in ids:
        if x & bit:
            a ^= x
    b = t ^ a
    return [a, b] if a < b else [b, a]
