import sys


def main() -> None:
    tokens = sys.stdin.buffer.read().split()
    n = int(tokens[0])
    seen: set[int] = set()
    distinct = [0] * (n + 1)
    for p in range(1, n + 1):
        seen.add(int(tokens[p]))
        distinct[p] = len(seen)
    q = int(tokens[1 + n])
    out = [str(distinct[int(x)]) for x in tokens[2 + n : 2 + n + q]]
    sys.stdout.write("\n".join(out) + "\n")


main()
