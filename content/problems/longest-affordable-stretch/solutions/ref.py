def longest_stretch(costs: list[int], budget: int) -> int:
    best = 0
    left = 0
    total = 0
    for right, c in enumerate(costs):
        total += c
        while total > budget:
            total -= costs[left]
            left += 1
        best = max(best, right - left + 1)
    return best
