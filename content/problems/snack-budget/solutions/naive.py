def most_snacks(prices: list[int], budget: int) -> int:
    # Correct but O(n²): scan for the cheapest snack still on the shelf, again and again.
    left = list(prices)
    count = 0
    while left:
        i = min(range(len(left)), key=left.__getitem__)
        if left[i] > budget:
            break
        budget -= left[i]
        left[i] = left[-1]
        left.pop()
        count += 1
    return count
