def longest_repeat(text: str) -> int:
    # Correct but O(n² · L): compare every pair of starting positions letter by letter.
    n, best = len(text), 0
    for i in range(n):
        for j in range(i + 1, n):
            k = 0
            while j + k < n and text[i + k] == text[j + k]:
                k += 1
            best = max(best, k)
    return best
