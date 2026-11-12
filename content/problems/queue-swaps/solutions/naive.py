def swaps_to_sort(heights: list[int]) -> int:
    # Correct but O(n²): count every pair that is out of order.
    n = len(heights)
    count = 0
    for i in range(n):
        h = heights[i]
        for j in range(i + 1, n):
            if h > heights[j]:
                count += 1
    return count
