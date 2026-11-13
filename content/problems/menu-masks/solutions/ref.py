def menu_combos(prices: list[int], budget: int) -> list[list[int]]:
    n = len(prices)
    out = []
    for mask in range(1, 1 << n):
        dishes = [i for i in range(n) if mask >> i & 1]
        if sum(prices[i] for i in dishes) == budget:
            out.append(dishes)
    return sorted(out)
