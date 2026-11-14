def quick_sort_3way(a, lo, hi):
    if lo >= hi:  # @step base
        return
    pivot = a[lo]  # @step pivot
    lt, i, gt = lo, lo + 1, hi
    while i <= gt:
        if a[i] < pivot:  # @step compare
            a[lt], a[i] = a[i], a[lt]  # @step less
            lt += 1
            i += 1
        elif a[i] > pivot:
            a[i], a[gt] = a[gt], a[i]  # @step greater
            gt -= 1
        else:
            i += 1  # @step equal
    quick_sort_3way(a, lo, lt - 1)
    quick_sort_3way(a, gt + 1, hi)
