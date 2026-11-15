# Correct but slow: walk up from b for every question, O(depth) each.
import sys


def main() -> None:
    data = sys.stdin.buffer.read().split()
    n = int(data[0])
    boss = [0, 0] + [int(x) for x in data[1:n]]
    q = int(data[n])
    out: list[str] = []
    pos = n + 1
    for _ in range(q):
        a, b = int(data[pos]), int(data[pos + 1])
        pos += 2
        x = b
        while x != 1 and x != a:
            x = boss[x]
        out.append("YES" if x == a and a != b else "NO")
    sys.stdout.write("\n".join(out) + "\n")


main()
