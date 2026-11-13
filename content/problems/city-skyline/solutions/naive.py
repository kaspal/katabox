def skyline(buildings: list[list[int]]) -> list[list[int]]:
    # Correct but O(n²): at every critical position, scan every building.
    xs = sorted({x for b in buildings for x in (b[0], b[1])})
    out, last = [], 0
    for x in xs:
        h = 0
        for left, right, height in buildings:
            if left <= x < right and height > h:
                h = height
        if h != last:
            out.append([x, h])
            last = h
    return out
