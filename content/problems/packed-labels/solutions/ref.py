import re
import sys
from bisect import bisect_left
from itertools import accumulate

RUN = re.compile(rb"(\d+)([a-z])")


def main() -> None:
    tokens = sys.stdin.buffer.read().split()
    n_labels = int(tokens[0])
    ends: list[list[int]] = []
    letters: list[str] = []
    for label in tokens[1 : 1 + n_labels]:
        runs = RUN.findall(label)
        ends.append(list(accumulate(int(count) for count, _ in runs)))
        letters.append(b"".join(ch for _, ch in runs).decode())
    q = int(tokens[1 + n_labels])
    pos = 2 + n_labels
    out: list[str] = []
    for _ in range(q):
        i, k = int(tokens[pos]) - 1, int(tokens[pos + 1])
        pos += 2
        out.append(letters[i][bisect_left(ends[i], k)])
    sys.stdout.write("\n".join(out) + "\n")


main()
