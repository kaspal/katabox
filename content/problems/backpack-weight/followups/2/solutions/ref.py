def best_pack_huge(weights: list[int], values: list[int], capacity: int) -> int:
    # lightest[v]: the least weight that reaches a total worth of exactly v.
    total = sum(values)
    inf = float("inf")
    lightest = [inf] * (total + 1)
    lightest[0] = 0
    for w, v in zip(weights, values):
        for t in range(total, v - 1, -1):
            if lightest[t - v] + w < lightest[t]:
                lightest[t] = lightest[t - v] + w
    return max(t for t in range(total + 1) if lightest[t] <= capacity)
