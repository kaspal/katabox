def outfits(prices: list[list[int]], budget: int) -> int:
    groups = [sorted(g) for g in prices]
    # cheapest[c] = the least that categories c, c + 1, ... can cost together.
    cheapest = [0] * (len(groups) + 1)
    for c in range(len(groups) - 1, -1, -1):
        cheapest[c] = cheapest[c + 1] + groups[c][0]

    def choose(c: int, spent: int) -> int:
        if c == len(groups):
            return 1
        total = 0
        for price in groups[c]:
            if spent + price + cheapest[c + 1] > budget:
                break
            total += choose(c + 1, spent + price)
        return total

    return choose(0, 0)
