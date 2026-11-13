from collections import Counter


def shortest_walk(stalls: str, need: str) -> int:
    # Obviously correct (short markets): every stretch, shortest first.
    want = Counter(need)
    n = len(stalls)
    for length in range(1, n + 1):
        for i in range(n - length + 1):
            have = Counter(stalls[i:i + length])
            if all(have[c] >= k for c, k in want.items()):
                return length
    return -1
