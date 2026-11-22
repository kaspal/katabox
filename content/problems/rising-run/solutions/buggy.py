from bisect import bisect_right


def rising_run(heights: list[int]) -> int:
    tails = []
    for h in heights:
        i = bisect_right(tails, h)
        if i == len(tails):
            tails.append(h)
        else:
            tails[i] = h
    return len(tails)
