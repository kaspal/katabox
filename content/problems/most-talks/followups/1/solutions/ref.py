from bisect import bisect_right


def best_programme(talks: list[list[int]]) -> int:
    order = sorted(talks, key=lambda t: t[1])
    ends = [t[1] for t in order]
    best = [0] * (len(order) + 1)  # best[i]: the best total using the first i talks by end
    for i, (s, e, score) in enumerate(order):
        j = bisect_right(ends, s, 0, i)  # talks ending at or before this start
        best[i + 1] = max(best[i], best[j] + score)
    return best[-1]
