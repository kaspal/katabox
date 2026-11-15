# Correct but slow: collect each stretch's species, O(n) per question.
import sys


def main() -> None:
    data = sys.stdin.buffer.read().split()
    n = int(data[0])
    ids = data[1 : 1 + n]
    q = int(data[1 + n])
    out: list[str] = []
    pos = 2 + n
    for _ in range(q):
        l, r = int(data[pos]), int(data[pos + 1])
        pos += 2
        seen = set()
        for i in range(l - 1, r):
            seen.add(ids[i])
        out.append(str(len(seen)))
    sys.stdout.write("\n".join(out) + "\n")


main()
