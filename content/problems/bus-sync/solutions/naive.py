# Correct but slow: walk the departures of the line with the longest period and stop at the
# first one every other line shares, O(lcm / max m · n) per timetable.
import sys
from math import gcd


def main() -> None:
    data = sys.stdin.buffer.read().split()
    t = int(data[0])
    pos = 1
    out: list[str] = []
    for _ in range(t):
        n = int(data[pos])
        lines = [(int(data[pos + 1 + 2 * i]), int(data[pos + 2 + 2 * i])) for i in range(n)]
        pos += 1 + 2 * n
        total = 1
        for _, m in lines:
            total = total // gcd(total, m) * m
        first, period = max(lines, key=lambda line: line[1])
        answer = -1
        for minute in range(first, total, period):
            if all(minute % m == r for r, m in lines):
                answer = minute
                break
        out.append(str(answer))
    sys.stdout.write("\n".join(out) + "\n")


main()
