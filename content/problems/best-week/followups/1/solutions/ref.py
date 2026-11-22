from collections import deque


def best_run(profits: list[int], k: int) -> int:
    n = len(profits)
    prefix = [0] * (n + 1)
    for i, p in enumerate(profits):
        prefix[i + 1] = prefix[i] + p
    # For each end j, the best start i in [j - k, j - 1] has the smallest prefix[i].
    best = None
    lows: deque[int] = deque()  # indexes into prefix, increasing prefix values
    for j in range(1, n + 1):
        i = j - 1
        while lows and prefix[lows[-1]] >= prefix[i]:
            lows.pop()
        lows.append(i)
        if lows[0] < j - k:
            lows.popleft()
        run = prefix[j] - prefix[lows[0]]
        best = run if best is None else max(best, run)
    return best
