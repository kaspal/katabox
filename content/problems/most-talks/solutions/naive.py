def most_talks(talks: list[list[int]]) -> int:
    # Correct but O(n²): sorted by end, best[i] = the most talks ending with talk i,
    # scanning every earlier talk.
    t = sorted(talks, key=lambda x: x[1])
    best = [1] * len(t)
    for i in range(len(t)):
        for j in range(i):
            if t[j][1] <= t[i][0] and best[j] + 1 > best[i]:
                best[i] = best[j] + 1
    return max(best)
