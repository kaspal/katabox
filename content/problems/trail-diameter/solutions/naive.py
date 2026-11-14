import sys


# Correct but O(N²): a BFS from every junction, keeping the largest distance seen.
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
        best = 0
        for start in range(1, n + 1):
            dist = [-1] * (n + 1)
            dist[start] = 0
            queue = [start]
            for v in queue:
                d = dist[v] + 1
                for w in adj[v]:
                    if dist[w] < 0:
                        dist[w] = d
                        queue.append(w)
            best = max(best, dist[queue[-1]])
        out.append(str(best))
    sys.stdout.write("\n".join(out) + "\n")


main()
