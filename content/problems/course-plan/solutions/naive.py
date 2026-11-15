# Correct but slow: every term, check every course left, O(terms · (n + m)).
import sys


def main() -> None:
    data = sys.stdin.buffer.read().split()
    n, m = int(data[0]), int(data[1])
    needs: list[list[int]] = [[] for _ in range(n + 1)]
    for i in range(m):
        a, b = int(data[2 + 2 * i]), int(data[3 + 2 * i])
        needs[b].append(a)
    passed = [False] * (n + 1)
    left = n
    terms: list[list[int]] = []
    while left > 0:
        ready = [c for c in range(1, n + 1) if not passed[c] and all(passed[a] for a in needs[c])]
        if not ready:
            sys.stdout.write("IMPOSSIBLE\n")
            return
        for c in ready:
            passed[c] = True
        left -= len(ready)
        terms.append(ready)
    out = [str(len(terms))] + [" ".join(map(str, t)) for t in terms]
    sys.stdout.write("\n".join(out) + "\n")


main()
