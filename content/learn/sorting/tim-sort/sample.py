RUN = 4


def tim_sort(a):
    n = len(a)
    for start in range(0, n, RUN):
        insertion_sort(a, start, min(start + RUN, n) - 1)  # @step run
    width = RUN
    while width < n:
        for lo in range(0, n - width, 2 * width):
            mid = lo + width - 1
            hi = min(lo + 2 * width, n) - 1
            merge(a, lo, mid, hi)  # @step merge
        width *= 2


def insertion_sort(a, lo, hi):
    for i in range(lo + 1, hi + 1):
        key = a[i]  # @step take
        j = i - 1
        while j >= lo and a[j] > key:  # @step compare
            a[j + 1] = a[j]  # @step shift
            j -= 1
        a[j + 1] = key  # @step insert


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
