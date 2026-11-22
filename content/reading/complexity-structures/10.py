def f(n):
    heap = [(i * 37) % n for i in range(n)]
    for start in range(n // 2 - 1, -1, -1):
        sift_down(heap, start, n)
    for end in range(n - 1, 0, -1):
        heap[0], heap[end] = heap[end], heap[0]
        sift_down(heap, 0, end)
    return heap

def sift_down(heap, i, size):
    while 2 * i + 1 < size:
        child = 2 * i + 1
        if child + 1 < size and heap[child + 1] > heap[child]:
            child += 1
        if heap[i] >= heap[child]:
            return
        heap[i], heap[child] = heap[child], heap[i]
        i = child
