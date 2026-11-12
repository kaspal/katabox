def window_swings(readings: list[int], k: int) -> list[int]:
    # Correct but O(n·k): scan every window for its highest and lowest reading.
    out = []
    for i in range(len(readings) - k + 1):
        hi = lo = readings[i]
        for j in range(i + 1, i + k):
            r = readings[j]
            if r > hi:
                hi = r
            if r < lo:
                lo = r
        out.append(hi - lo)
    return out
