def skyline(buildings: list[list[int]]) -> list[list[int]]:
    # Obviously correct (small coordinates): the outline height of every unit cell.
    end = max(b[1] for b in buildings)
    height = [0] * (end + 1)
    for left, right, h in buildings:
        for x in range(left, right):
            height[x] = max(height[x], h)
    out, last = [], 0
    for x, h in enumerate(height):
        if h != last:
            out.append([x, h])
            last = h
    return out
