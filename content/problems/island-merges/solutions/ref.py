import sys


def main() -> None:
    data = sys.stdin.buffer.read().split()
    r, c, q = int(data[0]), int(data[1]), int(data[2])
    width = c + 2  # a border of sea around the map: no bounds checks
    parent = [-1] * ((r + 2) * width)  # -1: sea; otherwise union-find over land squares

    def root(x: int) -> int:
        while parent[x] != x:
            parent[x] = parent[parent[x]]
            x = parent[x]
        return x

    islands = 0
    out: list[str] = []
    for k in range(q):
        cell = int(data[3 + 2 * k]) * width + int(data[4 + 2 * k])
        if parent[cell] == -1:
            # A new island, then one fewer for every different island it touches.
            parent[cell] = cell
            islands += 1
            for nxt in (cell + 1, cell - 1, cell + width, cell - width):
                if parent[nxt] != -1:
                    a, b = root(cell), root(nxt)
                    if a != b:
                        parent[b] = a
                        islands -= 1
        out.append(str(islands))
    sys.stdout.write("\n".join(out) + "\n")


main()
