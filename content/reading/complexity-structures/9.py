def f(n):
    heap = [(i * 37) % n for i in range(n)]
    for start in range(n // 2 - 1, -1, -1):
        sift_down(heap, start, n)
    return heap[0]

def sift_down(heap, i, size):
    while 2 * i + 1 < size:
        child = 2 * i + 1
        if child + 1 < size and heap[child + 1] > heap[child]:
            child += 1
        if heap[i] >= heap[child]:
            return
        heap[i], heap[child] = heap[child], heap[i]
        i = child
