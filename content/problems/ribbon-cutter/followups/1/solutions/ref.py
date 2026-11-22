from bisect import bisect_left


def longest_pieces(ribbons: list[int], orders: list[int]) -> list[int]:
    m = max(ribbons)
    n = len(ribbons)
    at_most = [0] * (m + 1)  # ribbons of length <= v
    for r in ribbons:
        at_most[r] += 1
    for v in range(1, m + 1):
        at_most[v] += at_most[v - 1]
    # count[L] = pieces of length L = sum over k >= 1 of (ribbons >= k*L).
    count = [0] * (m + 1)
    for length in range(1, m + 1):
        total = 0
        k = length
        while k <= m:
            total += n - at_most[k - 1]
            k += length
        count[length] = total
    # count is non-increasing in L; neg is non-decreasing for bisect.
    neg = [-c for c in count[1:]]
    out = []
    for need in orders:
        # The number of lengths with count >= need: the first index where -count > -need.
        k = bisect_left(neg, -need + 1)
        out.append(k)
    return out
