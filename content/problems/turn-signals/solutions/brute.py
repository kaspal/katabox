def turn_signals(routes: list[list[int]]) -> list[str]:
    # Obviously correct: the signed area of triangle ABC (shoelace) has the turn's sign.
    out = []
    for ax, ay, bx, by, cx, cy in routes:
        area2 = ax * (by - cy) + bx * (cy - ay) + cx * (ay - by)
        out.append("left" if area2 > 0 else "right" if area2 < 0 else "straight")
    return out
