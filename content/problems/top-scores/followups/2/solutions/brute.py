def kth_in_window(scores: list[int], w: int, k: int) -> list[int]:
    # Obviously correct: sort every window.
    return [sorted(scores[i : i + w], reverse=True)[k - 1] for i in range(len(scores) - w + 1)]
