def tilings_long(n: int) -> int:
    # Obviously correct for the small sizes the stress test uses: one step at a time.
    a, b, c = 1, 0, 0  # f(k), f(k-1), f(k-2)
    for _ in range(n):
        a, b, c = (a + b + c) % 1_000_003, a, b
    return a
