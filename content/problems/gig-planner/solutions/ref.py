import sys
from bisect import bisect_right


def main() -> None:
    data = sys.stdin.buffer.read().split()
    n = int(data[0])
    gigs = sorted((int(data[2 + 3 * i]), int(data[1 + 3 * i]), int(data[3 + 3 * i])) for i in range(n))  # by end
    ends = [e for e, _, _ in gigs]
    # best[i]: the best plan using only the first i offers (by end), as (total, -count):
    # a higher total wins, then fewer gigs. Either offer i is skipped, or it is taken
    # after the best plan of the offers that end by its start (a binary search).
    best = [(0, 0)] * (n + 1)
    for i, (_, start, pay) in enumerate(gigs):
        j = bisect_right(ends, start, 0, i)
        total, minus_count = best[j]
        best[i + 1] = max(best[i], (total + pay, minus_count - 1))
    total, minus_count = best[n]
    print(total, -minus_count)


main()
