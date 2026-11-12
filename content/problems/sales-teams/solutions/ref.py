def team_totals(boss: list[int], sales: list[int]) -> list[int]:
    children: list[list[int]] = [[] for _ in boss]
    owner = 0
    for e, b in enumerate(boss):
        if b < 0:
            owner = e
        else:
            children[b].append(e)
    order = [owner]
    for e in order:  # the list grows while we walk it: a breadth-first order
        order.extend(children[e])
    total = list(sales)
    for e in reversed(order):
        if boss[e] >= 0:
            total[boss[e]] += total[e]
    return total
