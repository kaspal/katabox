def exact_postage(values: list[int], target: int) -> list[int]:
    # Obviously correct: every subset, keeping the shortest, then smallest, index list.
    n = len(values)
    best = None
    for mask in range(1, 1 << n):
        idx = [i for i in range(n) if mask >> i & 1]
        if sum(values[i] for i in idx) == target:
            if best is None or (len(idx), idx) < (len(best), best):
                best = idx
    return best or []
