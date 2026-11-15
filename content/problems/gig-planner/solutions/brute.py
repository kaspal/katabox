# Obviously correct: every set of offers; keep the sets without overlaps (touching is
# fine), the highest total, then the fewest gigs.
from itertools import combinations


def main() -> None:
    n = int(input())
    gigs = [tuple(map(int, input().split())) for _ in range(n)]
    best = (0, 0)  # (total, -count)
    for size in range(1, n + 1):
        for chosen in combinations(gigs, size):
            ordered = sorted(chosen)
            if all(a[1] <= b[0] for a, b in zip(ordered, ordered[1:])):
                best = max(best, (sum(p for _, _, p in chosen), -size))
    print(best[0], -best[1])


main()
