import sys
from math import gcd


def main() -> None:
    data = sys.stdin.buffer.read().split()
    n = int(data[0])
    pts = [(int(data[1 + 2 * i]), int(data[2 + 2 * i])) for i in range(n)]
    twice = 0  # twice the area: the shoelace formula, in whole numbers
    boundary = 0  # lattice points on the edges: an edge (dx, dy) holds gcd(|dx|, |dy|)
    for i in range(n):
        ax, ay = pts[i]
        bx, by = pts[(i + 1) % n]
        twice += ax * by - bx * ay
        boundary += gcd(abs(bx - ax), abs(by - ay))
    twice = abs(twice)
    # Pick's theorem: area = inside + boundary / 2 - 1, so inside = (2·area - boundary + 2) / 2.
    inside = (twice - boundary + 2) // 2
    print(f"{twice // 2}.{5 if twice % 2 else 0} {inside}")


main()
