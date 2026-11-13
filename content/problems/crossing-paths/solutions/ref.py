def paths_cross(pairs: list[list[int]]) -> list[bool]:
    def orient(ax, ay, bx, by, cx, cy):
        c = (bx - ax) * (cy - ay) - (by - ay) * (cx - ax)
        return (c > 0) - (c < 0)

    def within(ax, ay, bx, by, px, py):
        return min(ax, bx) <= px <= max(ax, bx) and min(ay, by) <= py <= max(ay, by)

    out = []
    for x1, y1, x2, y2, x3, y3, x4, y4 in pairs:
        d1 = orient(x1, y1, x2, y2, x3, y3)
        d2 = orient(x1, y1, x2, y2, x4, y4)
        d3 = orient(x3, y3, x4, y4, x1, y1)
        d4 = orient(x3, y3, x4, y4, x2, y2)
        meet = (d1 * d2 < 0 and d3 * d4 < 0) or (
            (d1 == 0 and within(x1, y1, x2, y2, x3, y3))
            or (d2 == 0 and within(x1, y1, x2, y2, x4, y4))
            or (d3 == 0 and within(x3, y3, x4, y4, x1, y1))
            or (d4 == 0 and within(x3, y3, x4, y4, x2, y2))
        )
        out.append(meet)
    return out
