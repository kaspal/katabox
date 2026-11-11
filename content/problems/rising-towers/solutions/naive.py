def longest_rise(heights: list[int]) -> int:
    # Correct but O(n²): best[i] from every earlier tower.
    n = len(heights)
    best = [1] * n
    for i in range(n):
        h = heights[i]
        for j in range(i):
            if heights[j] < h and best[j] + 1 > best[i]:
                best[i] = best[j] + 1
    return max(best)
