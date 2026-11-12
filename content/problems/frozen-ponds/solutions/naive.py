def pond_sizes(ice: list[str]) -> list[int]:
    # Correct but slow: every frozen cell copies the smallest label among its frozen
    # neighbours, round after round, until nothing changes. A winding pond needs one
    # round per cell along it: O((r·c)²).
    h, w = len(ice), len(ice[0])
    label = [[r * w + c if ice[r][c] == "#" else -1 for c in range(w)] for r in range(h)]
    changed = True
    while changed:
        changed = False
        for r in range(h):
            for c in range(w):
                if label[r][c] < 0:
                    continue
                best = label[r][c]
                for y, x in ((r - 1, c), (r + 1, c), (r, c - 1), (r, c + 1)):
                    if 0 <= y < h and 0 <= x < w and 0 <= label[y][x] < best:
                        best = label[y][x]
                if best < label[r][c]:
                    label[r][c] = best
                    changed = True
    counts = {}
    for r in range(h):
        for c in range(w):
            if label[r][c] >= 0:
                counts[label[r][c]] = counts.get(label[r][c], 0) + 1
    return sorted(counts.values(), reverse=True)
