def selection_sort(a):
    n = len(a)
    for i in range(n - 1):
        smallest = i  # @step start
        for j in range(i + 1, n):
            if a[j] < a[smallest]:  # @step compare
                smallest = j  # @step new-min
        a[i], a[smallest] = a[smallest], a[i]  # @step swap
