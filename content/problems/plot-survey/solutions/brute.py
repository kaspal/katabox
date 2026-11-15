# Obviously correct: test every lattice point of the bounding box (boundary first, then
# an exact ray cast); the area from the shoelace formula, halved by hand.
def main() -> None:
    n = int(input())
    pts = [tuple(map(int, input().split())) for _ in range(n)]
    edges = [(pts[i], pts[(i + 1) % n]) for i in range(n)]

    def on_boundary(px, py):
        for (ax, ay), (bx, by) in edges:
            if (px - ax) * (by - ay) == (bx - ax) * (py - ay) and min(ax, bx) <= px <= max(ax, bx) and min(ay, by) <= py <= max(ay, by):
                return True
        return False

    def inside(px, py):
        crossings = 0
        for (ax, ay), (bx, by) in edges:
            if (ay > py) != (by > py):
                t = (px - ax) * (by - ay) - (bx - ax) * (py - ay)
                if (t < 0) == (by > ay):
                    crossings += 1
        return crossings % 2 == 1

    xs = [x for x, _ in pts]
    ys = [y for _, y in pts]
    count = 0
    for px in range(min(xs), max(xs) + 1):
        for py in range(min(ys), max(ys) + 1):
            if not on_boundary(px, py) and inside(px, py):
                count += 1
    twice = abs(sum(ax * by - bx * ay for (ax, ay), (bx, by) in edges))
    print(f"{twice // 2}.{5 if twice % 2 else 0}", count)


main()
