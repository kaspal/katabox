def counting_sort(a):
    counts = [0] * (max(a) + 1)
    for v in a:
        counts[v] += 1  # @step count
    k = 0
    for v in range(len(counts)):
        for _ in range(counts[v]):
            a[k] = v  # @step write
            k += 1
