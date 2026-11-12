def team_totals(boss: list[int], sales: list[int]) -> list[int]:
    # Correct but O(n · depth): add each employee's sales to every boss above them.
    total = list(sales)
    for e in range(len(boss)):
        b = boss[e]
        while b >= 0:
            total[b] += sales[e]
            b = boss[b]
    return total
