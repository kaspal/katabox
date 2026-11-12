def swaps_to_sort(heights: list[int]) -> int:
    # Obviously correct: bubble sort, counting the swaps of neighbours it makes.
    a = list(heights)
    swaps = 0
    changed = True
    while changed:
        changed = False
        for i in range(len(a) - 1):
            if a[i] > a[i + 1]:
                a[i], a[i + 1] = a[i + 1], a[i]
                swaps += 1
                changed = True
    return swaps
