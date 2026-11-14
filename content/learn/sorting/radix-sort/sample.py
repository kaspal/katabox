def radix_sort(a):
    place = 1
    while place <= max(a):  # @step pass
        buckets = [[] for _ in range(10)]
        for v in a:
            buckets[v // place % 10].append(v)  # @step distribute
        k = 0
        for bucket in buckets:
            for v in bucket:
                a[k] = v  # @step collect
                k += 1
        place *= 10
