import heapq


def huffman(freqs):
    heap = [(f, k) for k, f in enumerate(freqs)]
    heapq.heapify(heap)
    parent = {}
    made = len(freqs)
    while len(heap) > 1:
        fa, a = heapq.heappop(heap)
        fb, b = heapq.heappop(heap)
        parent[a] = parent[b] = made  # @step merge
        heapq.heappush(heap, (fa + fb, made))
        made += 1
    lengths = []
    for k in range(len(freqs)):
        depth, node = 0, k
        while node in parent:
            node = parent[node]
            depth += 1
        lengths.append(depth)
    bits = sum(f * d for f, d in zip(freqs, lengths))
    return [bits] + lengths
