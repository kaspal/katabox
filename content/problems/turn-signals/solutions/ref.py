def turn_signals(routes: list[list[int]]) -> list[str]:
    out = []
    for ax, ay, bx, by, cx, cy in routes:
        cross = (bx - ax) * (cy - by) - (by - ay) * (cx - bx)
        out.append("left" if cross > 0 else "right" if cross < 0 else "straight")
    return out
