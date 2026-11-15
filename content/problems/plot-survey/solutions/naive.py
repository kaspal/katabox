# Correct but slow: test every lattice point of the bounding box, O(area · n).
import sys


def main() -> None:
    data = sys.stdin.buffer.read().split()
    n = int(data[0])
    pts = [(int(data[1 + 2 * i]), int(data[2 + 2 * i])) for i in range(n)]
    edges = [(pts[i], pts[(i + 1) % n]) for i in range(n)]
    xs = [x for x, _ in pts]
    ys = [y for _, y in pts]
    count = 0
    for px in range(min(xs), max(xs) + 1):
        for py in range(min(ys), max(ys) + 1):
            crossings = 0
            boundary = False
            for (ax, ay), (bx, by) in edges:
                t = (px - ax) * (by - ay) - (bx - ax) * (py - ay)
                if t == 0 and min(ax, bx) <= px <= max(ax, bx) and min(ay, by) <= py <= max(ay, by):
                    boundary = True
                    break
                if (ay > py) != (by > py) and (t < 0) == (by > ay):
                    crossings += 1
            if not boundary and crossings % 2 == 1:
                count += 1
    twice = abs(sum(ax * by - bx * ay for (ax, ay), (bx, by) in edges))
    print(f"{twice // 2}.{5 if twice % 2 else 0} {count}")


main()
