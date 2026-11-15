# Correct but slow: from every start, extend while at most K sources, O(n²).
import sys


def main() -> None:
    data = sys.stdin.buffer.read().split()
    n, k = int(data[0]), int(data[1])
    ids = data[2 : 2 + n]
    count = 0
    for i in range(n):
        seen = set()
        for j in range(i, n):
            seen.add(ids[j])
            if len(seen) > k:
                break
            if len(seen) == k:
                count += 1
    print(count)


main()
