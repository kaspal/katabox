# Correct but slow: recount the islands over all land after every new square,
# O(Q · land).
import sys


def main() -> None:
    data = sys.stdin.buffer.read().split()
    r, c, q = int(data[0]), int(data[1]), int(data[2])
    width = c + 2
    land = bytearray((r + 2) * width)
    cells: list[int] = []
    out: list[str] = []
    for k in range(q):
        cell = int(data[3 + 2 * k]) * width + int(data[4 + 2 * k])
        if not land[cell]:
            land[cell] = 1
            cells.append(cell)
        seen = set()
        islands = 0
        for start in cells:
            if start in seen:
                continue
            islands += 1
            seen.add(start)
            stack = [start]
            while stack:
                cur = stack.pop()
                for nxt in (cur + 1, cur - 1, cur + width, cur - width):
                    if land[nxt] and nxt not in seen:
                        seen.add(nxt)
                        stack.append(nxt)
        out.append(str(islands))
    sys.stdout.write("\n".join(out) + "\n")


main()
