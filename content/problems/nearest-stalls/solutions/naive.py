def nearest_stalls(stalls: list[list[int]], k: int) -> list[int]:
    # Correct but O(n·k): select the nearest remaining stall k times.
    dist = [x * x + y * y for x, y in stalls]
    taken = [False] * len(stalls)
    out = []
    for _ in range(k):
        best = -1
        for i, d in enumerate(dist):
            if not taken[i] and (best < 0 or d < dist[best]):
                best = i
        taken[best] = True
        out.append(best)
    return out
