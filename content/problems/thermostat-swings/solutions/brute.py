def window_swings(readings: list[int], k: int) -> list[int]:
    # Obviously correct: max − min of every window.
    return [max(readings[i : i + k]) - min(readings[i : i + k]) for i in range(len(readings) - k + 1)]
