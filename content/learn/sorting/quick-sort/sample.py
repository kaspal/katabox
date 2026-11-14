def quick_sort(a, lo, hi):
    if lo >= hi:  # @step base
        return
    p = partition(a, lo, hi)
    quick_sort(a, lo, p - 1)
    quick_sort(a, p + 1, hi)


def partition(a, lo, hi):
    pivot = a[hi]  # @step pivot
    i = lo
    for j in range(lo, hi):
        if a[j] < pivot:  # @step compare
            a[i], a[j] = a[j], a[i]  # @step swap
            i += 1
    a[i], a[hi] = a[hi], a[i]  # @step place
    return i
