from itertools import accumulate


def prime_counts(limits: list[int]) -> list[int]:
    m = max(limits)
    sieve = bytearray([1]) * (m + 1)
    sieve[0] = 0
    if m >= 1:
        sieve[1] = 0
    p = 2
    while p * p <= m:
        if sieve[p]:
            sieve[p * p :: p] = bytes(len(range(p * p, m + 1, p)))
        p += 1
    count = list(accumulate(sieve))
    return [count[x] for x in limits]
