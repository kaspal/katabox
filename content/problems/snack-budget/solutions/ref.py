def most_snacks(prices: list[int], budget: int) -> int:
    spent = count = 0
    for p in sorted(prices):
        if spent + p > budget:
            break
        spent += p
        count += 1
    return count
