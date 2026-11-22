def pond_fences(ice: list[str]) -> list[int]:
    rows, cols = len(ice), len(ice[0])
    seen = [[False] * cols for _ in range(rows)]
    fences = []
    for r in range(rows):
        for c in range(cols):
            if ice[r][c] != "#" or seen[r][c]:
                continue
            seen[r][c] = True
            stack = [(r, c)]
            edges = 0
            while stack:
                y, x = stack.pop()
                for ny, nx in ((y + 1, x), (y - 1, x), (y, x + 1), (y, x - 1)):
                    if 0 <= ny < rows and 0 <= nx < cols and ice[ny][nx] == "#":
                        if not seen[ny][nx]:
                            seen[ny][nx] = True
                            stack.append((ny, nx))
                    else:
                        edges += 1  # a side facing open water, ground or the map's edge
            fences.append(edges)
    return sorted(fences, reverse=True)
