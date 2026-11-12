def users_with_all(users: list[int], queries: list[int]) -> list[int]:
    bits = max(max(users), max(queries), 1).bit_length()
    size = 1 << bits
    count = [0] * size
    for u in users:
        count[u] += 1
    for b in range(bits):
        bit = 1 << b
        for m in range(size):
            if not m & bit:
                count[m] += count[m | bit]
    return [count[q] for q in queries]
