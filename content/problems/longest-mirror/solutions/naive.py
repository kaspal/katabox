def longest_mirror(text: str) -> str:
    # Correct but O(n³): check every piece letter by letter.
    n = len(text)
    best_start, best_len = 0, 0
    for i in range(n):
        for j in range(i, n):
            lo, hi = i, j
            while lo < hi and text[lo] == text[hi]:
                lo += 1
                hi -= 1
            if lo >= hi and j - i + 1 > best_len:
                best_start, best_len = i, j - i + 1
    return text[best_start:best_start + best_len]
