import heapq


def skyline(buildings: list[list[int]]) -> list[list[int]]:
    order = sorted(buildings)
    xs = sorted({x for b in buildings for x in (b[0], b[1])})
    heap, out, i, last = [], [], 0, 0
    for x in xs:
        while i < len(order) and order[i][0] <= x:
            heapq.heappush(heap, (-order[i][2], order[i][1]))
            i += 1
        while heap and heap[0][1] <= x:
            heapq.heappop(heap)
        h = -heap[0][0] if heap else 0
        if h != last:
            out.append([x, h])
            last = h
    return out
