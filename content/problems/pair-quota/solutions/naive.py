# Correct but slow: every pair, O(n²) per case.
import sys


def main() -> None:
    data = sys.stdin.buffer.read().split()
    t = int(data[0])
    pos = 1
    out: list[str] = []
    for _ in range(t):
        n, k = int(data[pos]), int(data[pos + 1])
        values = list(map(int, data[pos + 2 : pos + 2 + n]))
        pos += 2 + n
        count = 0
        for i in range(n):
            a = values[i]
            for j in range(i + 1, n):
                if a + values[j] <= k:
                    count += 1
        out.append(str(count))
    sys.stdout.write("\n".join(out) + "\n")


main()
