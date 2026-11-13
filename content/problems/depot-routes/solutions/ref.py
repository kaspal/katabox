import sys
from heapq import heappop, heappush


def main() -> None:
    data = sys.stdin.buffer.read().split()
    n, m = int(data[0]), int(data[1])
    adj: list[list[tuple[int, int]]] = [[] for _ in range(n + 1)]
    pos = 2
    for _ in range(m):
        a, b, w = int(data[pos]), int(data[pos + 1]), int(data[pos + 2])
        pos += 3
        adj[a].append((b, w))
        adj[b].append((a, w))
    inf = 1 << 60
    dist = [inf] * (n + 1)
    dist[1] = 0
    heap = [(0, 1)]
    while heap:
        d, u = heappop(heap)
        if d > dist[u]:
            continue  # a stale entry
        for v, w in adj[u]:
            nd = d + w
            if nd < dist[v]:
                dist[v] = nd
                heappush(heap, (nd, v))
    q = int(data[pos])
    out: list[str] = []
    for i in range(pos + 1, pos + 1 + q):
        d = dist[int(data[i])]
        out.append("-1" if d == inf else str(d))
    sys.stdout.write("\n".join(out) + "\n")


main()
