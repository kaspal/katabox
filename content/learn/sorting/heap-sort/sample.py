def heap_sort(a):
    n = len(a)
    for start in range(n // 2 - 1, -1, -1):
        sift_down(a, start, n)  # @step build
    for end in range(n - 1, 0, -1):
        a[0], a[end] = a[end], a[0]  # @step extract
        sift_down(a, 0, end)


def sift_down(a, root, size):
    child = 2 * root + 1
    while child < size:
        if child + 1 < size and a[child + 1] > a[child]:  # @step pick
            child += 1
        if a[root] >= a[child]:  # @step compare
            return
        a[root], a[child] = a[child], a[root]  # @step swap
        root = child
        child = 2 * root + 1
