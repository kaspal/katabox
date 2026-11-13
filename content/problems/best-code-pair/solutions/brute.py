def best_xor_pair(codes: list[int]) -> int:
    # Obviously correct: every pair.
    return max(codes[i] ^ codes[j] for i in range(len(codes)) for j in range(i + 1, len(codes)))
