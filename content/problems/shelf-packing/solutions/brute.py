def fewest_shelves(widths: list[int], width: int) -> int:
    # Obviously correct for small inputs: every way to split the books into groups
    # (each book joins an existing group or starts a new one), keeping those that fit.
    best = len(widths)

    def assign(i, groups):
        nonlocal best
        if i == len(widths):
            if all(g <= width for g in groups):
                best = min(best, len(groups))
            return
        for k in range(len(groups)):
            groups[k] += widths[i]
            assign(i + 1, groups)
            groups[k] -= widths[i]
        groups.append(widths[i])
        assign(i + 1, groups)
        groups.pop()

    assign(0, [])
    return best
