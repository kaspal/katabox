def largest_poster(panels: list[int]) -> int:
    # Obviously correct: every stretch of panels, with its lowest panel.
    n = len(panels)
    return max(min(panels[i:j]) * (j - i) for i in range(n) for j in range(i + 1, n + 1))
