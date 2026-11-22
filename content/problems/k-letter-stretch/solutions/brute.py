def longest_with_k_letters(text: str, k: int) -> int:
    # Obviously correct: every stretch, counting its different letters.
    best = 0
    for i in range(len(text)):
        for j in range(i, len(text)):
            if len(set(text[i : j + 1])) <= k:
                best = max(best, j - i + 1)
    return best
