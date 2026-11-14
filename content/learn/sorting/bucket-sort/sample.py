def bucket_sort(a, count=5):
    buckets = [[] for _ in range(count)]
    for v in a:
        buckets[v * count // 100].append(v)  # @step scatter
    k = 0
    for bucket in buckets:
        bucket.sort()  # @step sort
        for v in bucket:
            a[k] = v  # @step gather
            k += 1
