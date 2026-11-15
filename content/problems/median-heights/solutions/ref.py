import sys


def main() -> None:
    data = sys.stdin.buffer.read().split()
    t = int(data[0])
    pos = 1
    out: list[str] = []
    for _ in range(t):
        n = int(data[pos])
        heights = sorted(map(int, data[pos + 1 : pos + 1 + n]))
        pos += 1 + n
        # The middle singer, or the lower of the two middle ones when n is even.
        out.append(str(heights[(n - 1) // 2]))
    sys.stdout.write("\n".join(out) + "\n")


main()
