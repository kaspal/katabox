def best_pack(weights: list[int], values: list[int], capacity: int) -> int:
    # Correct but O(2ⁿ): take or skip each item recursively, without remembering anything.
    def go(i, room):
        if i == len(weights):
            return 0
        best = go(i + 1, room)
        if weights[i] <= room:
            best = max(best, values[i] + go(i + 1, room - weights[i]))
        return best

    return go(0, capacity)
