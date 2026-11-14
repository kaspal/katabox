def push(heap, value):
    heap.append(value)  # @step append
    i = len(heap) - 1
    while i > 0:
        parent = (i - 1) // 2
        if heap[parent] <= heap[i]:  # @step compare-up
            break
        heap[parent], heap[i] = heap[i], heap[parent]  # @step swap-up
        i = parent


def pop(heap):
    if not heap:
        return -1  # @step empty
    top = heap[0]  # @step take
    last = heap.pop()
    if heap:
        heap[0] = last  # @step move-last
        i = 0
        while 2 * i + 1 < len(heap):
            child = 2 * i + 1
            if child + 1 < len(heap) and heap[child + 1] < heap[child]:
                child += 1
            if heap[i] <= heap[child]:  # @step compare-down
                break
            heap[i], heap[child] = heap[child], heap[i]  # @step swap-down
            i = child
    return top


def run(ops):
    heap, out = [], []
    for op in ops:
        if op[0] == "push":
            push(heap, op[1])
        else:
            out.append(pop(heap))
    return out
