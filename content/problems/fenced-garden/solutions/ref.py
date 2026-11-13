def where_is_point(fence: list[list[int]], points: list[list[int]]) -> list[str]:
    n = len(fence)
    out = []
    for px, py in points:
        on_fence, inside = False, False
        for i in range(n):
            ax, ay = fence[i]
            bx, by = fence[(i + 1) % n]
            if (bx - ax) * (py - ay) - (by - ay) * (px - ax) == 0 and min(ax, bx) <= px <= max(ax, bx) and min(ay, by) <= py <= max(ay, by):
                on_fence = True
                break
            if (ay > py) != (by > py):
                t = (py - ay) * (bx - ax) - (px - ax) * (by - ay)
                if (t > 0) == (by > ay):
                    inside = not inside
        out.append("boundary" if on_fence else "inside" if inside else "outside")
    return out
