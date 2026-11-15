# Correct but slow: best fee first, searching back day by day for a free day, O(n²).
import sys


def main() -> None:
    data = sys.stdin.buffer.read().split()
    n = int(data[0])
    jobs = sorted(((int(data[2 + 2 * i]), int(data[1 + 2 * i])) for i in range(n)), reverse=True)
    taken = [False] * (n + 1)
    total = 0
    for fee, deadline in jobs:
        day = min(deadline, n)
        while day > 0 and taken[day]:
            day -= 1
        if day > 0:
            taken[day] = True
            total += fee
    print(total)


main()
