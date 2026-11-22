def rising_run(heights: list[int]) -> int:
    # Obviously correct: best[i] = longest strictly rising pick ending at i.
    best = []
    for i, h in enumerate(heights):
        best.append(1 + max((best[j] for j in range(i) if heights[j] < h), default=0))
    return max(best)
