def suffix_array(text):
    n = len(text)
    rank = [ord(ch) for ch in text]
    order = list(range(n))
    k = 1
    while True:
        def key(i):
            return (rank[i], rank[i + k] if i + k < n else -1)
        order.sort(key=key)  # @step sort
        new = [0] * n
        for j in range(1, n):
            new[order[j]] = new[order[j - 1]] + (key(order[j]) != key(order[j - 1]))
        rank = new
        if rank[order[-1]] == n - 1:
            return order
        k *= 2  # @step double
