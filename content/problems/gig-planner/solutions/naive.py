# Correct but slow: the best plan ending with each offer, trying every earlier offer, O(n²).
import sys


def main() -> None:
    data = sys.stdin.buffer.read().split()
    n = int(data[0])
    gigs = sorted((int(data[2 + 3 * i]), int(data[1 + 3 * i]), int(data[3 + 3 * i])) for i in range(n))  # by end
    best_with: list[tuple[int, int]] = []  # (total, -count) of the best plan ending with gig i
    for i, (end, start, pay) in enumerate(gigs):
        before = (0, 0)
        for k in range(i):
            if gigs[k][0] <= start and best_with[k] > before:
                before = best_with[k]
        best_with.append((before[0] + pay, before[1] - 1))
    total, minus_count = max(best_with)
    print(total, -minus_count)


main()
