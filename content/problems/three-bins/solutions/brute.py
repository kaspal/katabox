def sort_bins(items: list[int]) -> list[int]:
    # Obviously correct: count each kind and write them out in order.
    return [0] * items.count(0) + [1] * items.count(1) + [2] * items.count(2)
