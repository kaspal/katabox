import sys


def farthest(adj: list[list[int]], start: int) -> tuple[int, int]:
    """Iterative BFS: the last junction reached is a farthest one."""
    dist = [-1] * len(adj)
    dist[start] = 0
    queue = [start]
    for v in queue:
        d = dist[v] + 1
        for w in adj[v]:
            if dist[w] < 0:
                dist[w] = d
                queue.append(w)
    last = queue[-1]
    return last, dist[last]


def main() -> None:
    tokens = sys.stdin.buffer.read().split()
    t = int(tokens[0])
    pos = 1
    out: list[str] = []
    for _ in range(t):
        n = int(tokens[pos])
        pos += 1
        adj: list[list[int]] = [[] for _ in range(n + 1)]
        for _ in range(n - 1):
            a, b = int(tokens[pos]), int(tokens[pos + 1])
            pos += 2
            adj[a].append(b)
            adj[b].append(a)
        u, _ = farthest(adj, 1)
        _, diameter = farthest(adj, u)
        out.append(str(diameter))
    sys.stdout.write("\n".join(out) + "\n")


main()
