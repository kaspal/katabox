def f(n):
    prefix = [0] * (n + 1)
    for i in range(n):
        prefix[i + 1] = prefix[i] + i % 10
    best = 0
    for q in range(2 * n):
        left = q % n
        right = min(n, left + q % 50 + 1)
        best = max(best, prefix[right] - prefix[left])
    return best
