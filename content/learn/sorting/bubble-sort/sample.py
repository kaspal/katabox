def bubble_sort(a):
    n = len(a)
    for end in range(n - 1, 0, -1):  # @step pass
        swapped = False
        for j in range(end):
            if a[j] > a[j + 1]:  # @step compare
                a[j], a[j + 1] = a[j + 1], a[j]  # @step swap
                swapped = True
        if not swapped:  # @step check
            break
    return a
