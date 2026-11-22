def kth_after_each(scores: list[int], k: int) -> list[int]:
    # Obviously correct: sort the scores so far after every game.
    out = []
    for i in range(1, len(scores) + 1):
        so_far = sorted(scores[:i], reverse=True)
        out.append(so_far[k - 1] if i >= k else -1)
    return out
