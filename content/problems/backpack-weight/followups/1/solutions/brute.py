def best_pack_copies(weights: list[int], values: list[int], counts: list[int], capacity: int) -> int:
    # Obviously correct for tiny inputs: try every number of copies of every item.
    def go(i: int, left: int) -> int:
        if i == len(weights):
            return 0
        best = 0
        for k in range(counts[i] + 1):
            if k * weights[i] > left:
                break
            best = max(best, k * values[i] + go(i + 1, left - k * weights[i]))
        return best

    return go(0, capacity)
