def merge_sort(a, lo, hi):
    if lo >= hi:  # @step base
        return
    mid = (lo + hi) // 2  # @step split
    merge_sort(a, lo, mid)
    merge_sort(a, mid + 1, hi)
    merge(a, lo, mid, hi)


def merge(a, lo, mid, hi):
    buffer = a[lo:hi + 1]  # @step copy
    i, j = 0, mid + 1 - lo
    for k in range(lo, hi + 1):
        if j > hi - lo or (i <= mid - lo and buffer[i] <= buffer[j]):  # @step pick
            a[k] = buffer[i]  # @step take-left
            i += 1
        else:
            a[k] = buffer[j]  # @step take-right
            j += 1
