def best_pack(weights: list[int], values: list[int], capacity: int) -> int:
    best = [0] * (capacity + 1)
    for w, v in zip(weights, values):
        for c in range(capacity, w - 1, -1):
            if best[c - w] + v > best[c]:
                best[c] = best[c - w] + v
    return best[capacity]
