# Correct but slow: a fresh Dijkstra from the depot for every question (stopping when the
# town is reached), O(Q·(N + M) log N).
import sys
from heapq import heappop, heappush


def search(adj: list[list[tuple[int, int]]], n: int, target: int) -> int:
    dist = [1 << 60] * (n + 1)
    dist[1] = 0
    heap = [(0, 1)]
    while heap:
        d, u = heappop(heap)
        if d > dist[u]:
            continue
        if u == target:
            return d
        for v, w in adj[u]:
            if d + w < dist[v]:
                dist[v] = d + w
                heappush(heap, (d + w, v))
    return -1


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
    q = int(data[pos])
    out = [str(search(adj, n, int(data[i]))) for i in range(pos + 1, pos + 1 + q)]
    sys.stdout.write("\n".join(out) + "\n")


main()
