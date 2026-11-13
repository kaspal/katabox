def sort_bins(items: list[int]) -> list[int]:
    low, mid, high = 0, 0, len(items) - 1
    while mid <= high:
        if items[mid] == 0:
            items[low], items[mid] = items[mid], items[low]
            low += 1
            mid += 1
        elif items[mid] == 1:
            mid += 1
        else:
            items[mid], items[high] = items[high], items[mid]
            high -= 1
    return items
