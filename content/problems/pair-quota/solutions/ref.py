import sys


def main() -> None:
    data = sys.stdin.buffer.read().split()
    t = int(data[0])
    pos = 1
    out: list[str] = []
    for _ in range(t):
        n, k = int(data[pos]), int(data[pos + 1])
        values = sorted(map(int, data[pos + 2 : pos + 2 + n]))
        pos += 2 + n
        # Two pointers on the sorted values: if the smallest and the largest fit, the
        # smallest fits with everything in between too.
        count = 0
        i, j = 0, n - 1
        while i < j:
            if values[i] + values[j] <= k:
                count += j - i
                i += 1
            else:
                j -= 1
        out.append(str(count))
    sys.stdout.write("\n".join(out) + "\n")


main()
