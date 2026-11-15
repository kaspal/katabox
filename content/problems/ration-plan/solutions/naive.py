# Correct but slow: walk the sorted prices for every budget, O(n·Q).
import sys


def main() -> None:
    data = sys.stdin.buffer.read().split()
    n, q = int(data[0]), int(data[1])
    prices = sorted(map(int, data[2 : 2 + n]))
    out: list[str] = []
    for b in data[2 + n : 2 + n + q]:
        budget = int(b)
        spent = 0
        count = 0
        for p in prices:
            if spent + p > budget:
                break
            spent += p
            count += 1
        out.append(str(count))
    sys.stdout.write("\n".join(out) + "\n")


main()
