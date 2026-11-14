# Correct but slow: scan every stretch, O(N·Q).
import sys


def main() -> None:
    data = sys.stdin.buffer.read().split()
    n = int(data[0])
    levels = list(map(int, data[1 : 1 + n]))
    q = int(data[1 + n])
    out: list[str] = []
    pos = 2 + n
    for _ in range(q):
        l, r = int(data[pos]), int(data[pos + 1])
        pos += 2
        out.append(str(min(levels[l - 1 : r])))
    sys.stdout.write("\n".join(out) + "\n")


main()
