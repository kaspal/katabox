def top_scores(scores: list[int], k: int) -> list[int]:
    # Obviously correct: take the highest remaining score k times.
    left = list(scores)
    out = []
    for _ in range(k):
        best = max(left)
        left.remove(best)
        out.append(best)
    return out
