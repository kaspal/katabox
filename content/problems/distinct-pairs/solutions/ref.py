def distinct_pairs(prices: list[int], budget: int) -> int:
    i, j = 0, len(prices) - 1
    count = 0
    while i < j:
        s = prices[i] + prices[j]
        if s < budget:
            i += 1
        elif s > budget:
            j -= 1
        else:
            count += 1
            left, right = prices[i], prices[j]
            while i < j and prices[i] == left:
                i += 1
            while i < j and prices[j] == right:
                j -= 1
    return count
