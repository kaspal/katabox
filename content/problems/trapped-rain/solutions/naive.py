def trapped_rain(columns: list[int]) -> int:
    # Correct but O(n²): scan both sides of every column for the tallest.
    water = 0
    for k, h in enumerate(columns):
        water += min(max(columns[:k + 1]), max(columns[k:])) - h
    return water
