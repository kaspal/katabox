from bisect import bisect_left


def rising_run(heights: list[int]) -> int:
    # tails[k] = the smallest last height of a strictly rising pick of length k + 1.
    tails = []
    for h in heights:
        i = bisect_left(tails, h)  # an equal height replaces, it never extends
        if i == len(tails):
            tails.append(h)
        else:
            tails[i] = h
    return len(tails)
