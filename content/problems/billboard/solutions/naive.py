def largest_poster(panels: list[int]) -> int:
    # Correct but O(n²): every left end, with a running minimum to the right.
    best = 0
    for i in range(len(panels)):
        low = panels[i]
        for j in range(i, len(panels)):
            low = min(low, panels[j])
            best = max(best, low * (j - i + 1))
    return best
