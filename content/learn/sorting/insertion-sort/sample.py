def insertion_sort(a):
    for i in range(1, len(a)):
        key = a[i]  # @step take
        j = i - 1
        while j >= 0 and a[j] > key:  # @step compare
            a[j + 1] = a[j]  # @step shift
            j -= 1
        a[j + 1] = key  # @step insert
    return a
