from bisect import bisect_left


def longest_rise(heights: list[int]) -> int:
    tails: list[int] = []
    for h in heights:
        p = bisect_left(tails, h)
        if p == len(tails):
            tails.append(h)
        else:
            tails[p] = h
    return len(tails)
