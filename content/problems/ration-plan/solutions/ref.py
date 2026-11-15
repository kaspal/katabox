import sys
from bisect import bisect_right
from itertools import accumulate


def main() -> None:
    data = sys.stdin.buffer.read().split()
    n, q = int(data[0]), int(data[1])
    # total[k - 1]: the cost of the k cheapest rations. It only grows, so the most
    # rations a budget buys is the number of totals within it.
    total = list(accumulate(sorted(map(int, data[2 : 2 + n]))))
    out = [str(bisect_right(total, int(b))) for b in data[2 + n : 2 + n + q]]
    sys.stdout.write("\n".join(out) + "\n")


main()
