import sys


def main() -> None:
    tokens = sys.stdin.buffer.read().split()
    r = int(tokens[0])
    rows = tokens[2 : 2 + r]
    row_trees = [row.count(b"#") for row in rows]
    col_trees = [column.count(35) for column in zip(*rows)]  # 35 is "#"
    q = int(tokens[2 + r])
    pos = 3 + r
    out: list[str] = []
    for _ in range(q):
        kind, index = tokens[pos], int(tokens[pos + 1])
        pos += 2
        out.append(str(row_trees[index] if kind == b"row" else col_trees[index]))
    sys.stdout.write("\n".join(out) + "\n")


main()
