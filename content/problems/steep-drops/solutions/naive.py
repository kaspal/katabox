def steep_drops(prices: list[int]) -> int:
    # Correct but O(n²): check every pair of days.
    count = 0
    for i, x in enumerate(prices):
        for y in prices[i + 1:]:
            if x > 2 * y:
                count += 1
    return count
