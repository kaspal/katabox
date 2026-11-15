import sys


def at_most(ids: list[bytes], m: int) -> int:
    """How many stretches have at most m different sources (a sliding window)."""
    if m == 0:
        return 0
    count: dict[bytes, int] = {}
    left = 0
    total = 0
    for right, x in enumerate(ids):
        count[x] = count.get(x, 0) + 1
        while len(count) > m:
            y = ids[left]
            count[y] -= 1
            if count[y] == 0:
                del count[y]
            left += 1
        # Every stretch ending at `right` and starting at left..right qualifies.
        total += right - left + 1
    return total


def main() -> None:
    data = sys.stdin.buffer.read().split()
    n, k = int(data[0]), int(data[1])
    ids = data[2 : 2 + n]
    # Exactly K = at most K minus at most K − 1.
    print(at_most(ids, k) - at_most(ids, k - 1))


main()
