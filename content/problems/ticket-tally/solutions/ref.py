import sys


def main() -> None:
    tokens = sys.stdin.buffer.read().split()
    t = int(tokens[0])
    pos = 1
    out: list[str] = []
    for _ in range(t):
        n = int(tokens[pos])
        prices = [int(x) for x in tokens[pos + 1 : pos + 1 + n]]
        pos += 1 + n
        out.append(f"{n} {sum(prices)} {max(prices, default=0)}")
    sys.stdout.write("\n".join(out) + "\n")


main()
