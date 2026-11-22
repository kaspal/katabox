def exact_baskets(weights: list[int], k: int, target: int) -> list[list[int]]:
    items = sorted(weights)
    out = []
    pick = []

    def go(i: int, total: int) -> None:
        if len(pick) == k:
            if total == target:
                out.append(pick[:])
            return
        for j in range(i, len(items)):
            pick.append(items[j])
            go(j + 1, total + items[j])
            pick.pop()  # undo the choice before trying the next one

    go(0, 0)
    return out
