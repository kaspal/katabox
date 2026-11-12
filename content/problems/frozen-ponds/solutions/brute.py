def pond_sizes(ice: list[str]) -> list[int]:
    # Obviously correct: a depth-first flood fill from every unvisited frozen cell.
    h, w = len(ice), len(ice[0])
    seen = set()
    sizes = []
    for r in range(h):
        for c in range(w):
            if ice[r][c] != "#" or (r, c) in seen:
                continue
            stack, n = [(r, c)], 0
            seen.add((r, c))
            while stack:
                y, x = stack.pop()
                n += 1
                for ny, nx in ((y - 1, x), (y + 1, x), (y, x - 1), (y, x + 1)):
                    if 0 <= ny < h and 0 <= nx < w and ice[ny][nx] == "#" and (ny, nx) not in seen:
                        seen.add((ny, nx))
                        stack.append((ny, nx))
            sizes.append(n)
    return sorted(sizes, reverse=True)
