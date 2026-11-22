def pond_fences(ice: list[str]) -> list[int]:
    # Obviously correct: label ponds by repeated relabelling, then count each frozen
    # cell's sides that face something not frozen.
    rows, cols = len(ice), len(ice[0])
    label = {(r, c): r * cols + c for r in range(rows) for c in range(cols) if ice[r][c] == "#"}
    changed = True
    while changed:
        changed = False
        for (r, c), l in list(label.items()):
            for n in ((r + 1, c), (r - 1, c), (r, c + 1), (r, c - 1)):
                if n in label and label[n] < label[(r, c)]:
                    label[(r, c)] = label[n]
                    changed = True
    fences: dict[int, int] = {}
    for (r, c), l in label.items():
        sides = sum(1 for n in ((r + 1, c), (r - 1, c), (r, c + 1), (r, c - 1)) if n not in label)
        fences[l] = fences.get(l, 0) + sides
    return sorted(fences.values(), reverse=True)
