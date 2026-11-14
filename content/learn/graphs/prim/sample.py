import heapq


def prim(n, adj):
    in_tree = [False] * n
    order, total = [], 0
    heap = [(0, 0, 0)]
    while heap:
        w, v, u = heapq.heappop(heap)  # @step pop
        if in_tree[v]:
            continue  # @step skip
        in_tree[v] = True
        order.append(v)
        total += w  # @step take
        for x, wx in adj[v]:
            if not in_tree[x]:
                heapq.heappush(heap, (wx, x, v))  # @step push
    return [total] + order
