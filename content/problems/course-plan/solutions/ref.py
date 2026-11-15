import sys


def main() -> None:
    data = sys.stdin.buffer.read().split()
    n, m = int(data[0]), int(data[1])
    unlocks: list[list[int]] = [[] for _ in range(n + 1)]
    missing = [0] * (n + 1)  # prerequisites not passed yet (a repeated rule counts twice)
    for i in range(m):
        a, b = int(data[2 + 2 * i]), int(data[3 + 2 * i])
        unlocks[a].append(b)
        missing[b] += 1
    # Kahn's algorithm, one term at a time: a term is every course whose last missing
    # prerequisite was passed in the term before.
    term = [c for c in range(1, n + 1) if missing[c] == 0]
    terms: list[list[int]] = []
    passed = 0
    while term:
        term.sort()
        terms.append(term)
        passed += len(term)
        following: list[int] = []
        for a in term:
            for b in unlocks[a]:
                missing[b] -= 1
                if missing[b] == 0:
                    following.append(b)
        term = following
    if passed < n:
        sys.stdout.write("IMPOSSIBLE\n")
        return
    out = [str(len(terms))] + [" ".join(map(str, t)) for t in terms]
    sys.stdout.write("\n".join(out) + "\n")


main()
