def team_totals(boss: list[int], sales: list[int]) -> list[int]:
    # Obviously correct: for each employee, add up everyone whose chain of bosses
    # passes through them (including themselves).
    n = len(boss)

    def chain(e):
        while e >= 0:
            yield e
            e = boss[e]

    return [sum(sales[x] for x in range(n) if e in chain(x)) for e in range(n)]
