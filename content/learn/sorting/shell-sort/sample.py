def shell_sort(a):
    gap = len(a) // 2
    while gap > 0:  # @step gap
        for i in range(gap, len(a)):
            key = a[i]  # @step take
            j = i
            while j >= gap and a[j - gap] > key:  # @step compare
                a[j] = a[j - gap]  # @step shift
                j -= gap
            a[j] = key  # @step insert
        gap //= 2
