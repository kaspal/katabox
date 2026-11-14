def prefix_sums(a):
    p = [0] * (len(a) + 1)
    for i in range(len(a)):
        p[i + 1] = p[i] + a[i]  # @step add
    return p


def range_sum(p, lo, hi):
    return p[hi + 1] - p[lo]  # @step query
