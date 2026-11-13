def best_xor_pair(codes: list[int]) -> int:
    # Correct but O(n²): try every pair.
    best = 0
    n = len(codes)
    for i in range(n):
        x = codes[i]
        for j in range(i + 1, n):
            v = x ^ codes[j]
            if v > best:
                best = v
    return best
