def count_pattern(text: str, pattern: str) -> int:
    # Correct but O(n · m): compare character by character at every position.
    n, m = len(text), len(pattern)
    count = 0
    for i in range(n - m + 1):
        k = 0
        while k < m and text[i + k] == pattern[k]:
            k += 1
        if k == m:
            count += 1
    return count
