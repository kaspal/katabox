import sys


def main() -> None:
    data = sys.stdin.buffer.read().split()
    n = int(data[0])
    ids = data[1 : 1 + n]
    q = int(data[1 + n])
    by_end: list[list[tuple[int, int]]] = [[] for _ in range(n + 1)]
    for k in range(q):
        l, r = int(data[2 + n + 2 * k]), int(data[3 + n + 2 * k])
        by_end[r].append((l, k))
    # Offline, by right end: while the log is read up to photo r, mark only the latest
    # photo of each species. Then the species of l..r are the marks in l..r. A Fenwick
    # tree keeps the marks' prefix counts.
    tree = [0] * (n + 1)
    latest: dict[bytes, int] = {}
    answer = [0] * q
    for r in range(1, n + 1):
        before = latest.get(ids[r - 1])
        if before is not None:
            i = before
            while i <= n:
                tree[i] -= 1
                i += i & -i
        latest[ids[r - 1]] = r
        i = r
        while i <= n:
            tree[i] += 1
            i += i & -i
        for l, k in by_end[r]:
            # marks in 1..r minus marks in 1..l-1
            total = 0
            i = r
            while i > 0:
                total += tree[i]
                i -= i & -i
            i = l - 1
            while i > 0:
                total -= tree[i]
                i -= i & -i
            answer[k] = total
    sys.stdout.write("\n".join(map(str, answer)) + "\n")


main()
