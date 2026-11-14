INF = 10 ** 6


def tsp(n, dist):
    full = 1 << n
    best = [[INF] * n for _ in range(full)]
    best[1][0] = 0
    for mask in range(1, full, 2):
        for last in range(n):
            if best[mask][last] == INF:
                continue
            for city in range(n):
                if mask & (1 << city):
                    continue
                nxt = mask | (1 << city)
                total = best[mask][last] + dist[last][city]
                if total < best[nxt][city]:
                    best[nxt][city] = total  # @step extend
    return [min(best[full - 1][last] + dist[last][0] for last in range(1, n))]
