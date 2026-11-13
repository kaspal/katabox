from fractions import Fraction


def paths_cross(pairs: list[list[int]]) -> list[bool]:
    # Obviously correct: solve P1 + t(P2 − P1) = P3 + u(P4 − P3) exactly with fractions;
    # for parallel paths, compare their spans along the shared line.
    out = []
    for x1, y1, x2, y2, x3, y3, x4, y4 in pairs:
        rx, ry, sx, sy = x2 - x1, y2 - y1, x4 - x3, y4 - y3
        den = rx * sy - ry * sx
        qx, qy = x3 - x1, y3 - y1
        if den != 0:
            t = Fraction(qx * sy - qy * sx, den)
            u = Fraction(qx * ry - qy * rx, den)
            out.append(0 <= t <= 1 and 0 <= u <= 1)
        elif qx * ry - qy * rx != 0:
            out.append(False)
        else:
            rr = rx * rx + ry * ry
            t0 = Fraction(qx * rx + qy * ry, rr)
            t1 = Fraction((x4 - x1) * rx + (y4 - y1) * ry, rr)
            out.append(max(min(t0, t1), 0) <= min(max(t0, t1), 1))
    return out
