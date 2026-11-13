import math


def where_is_point(fence: list[list[int]], points: list[list[int]]) -> list[str]:
    # Obviously correct: exact on-fence test, then the winding angle (2π inside, 0 outside).
    n = len(fence)
    out = []
    for px, py in points:
        boundary = False
        for i in range(n):
            ax, ay = fence[i]
            bx, by = fence[(i + 1) % n]
            if (bx - ax) * (py - ay) - (by - ay) * (px - ax) == 0 and min(ax, bx) <= px <= max(ax, bx) and min(ay, by) <= py <= max(ay, by):
                boundary = True
        if boundary:
            out.append("boundary")
            continue
        total = 0.0
        for i in range(n):
            a1 = math.atan2(fence[i][1] - py, fence[i][0] - px)
            a2 = math.atan2(fence[(i + 1) % n][1] - py, fence[(i + 1) % n][0] - px)
            d = a2 - a1
            while d > math.pi:
                d -= 2 * math.pi
            while d < -math.pi:
                d += 2 * math.pi
            total += d
        out.append("inside" if abs(total) > math.pi else "outside")
    return out
