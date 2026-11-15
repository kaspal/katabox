# Obviously correct: try every set of jobs; a set can all be done in time exactly when,
# taken in deadline order, the k-th job's deadline is at least k.
from itertools import combinations


def main() -> None:
    n = int(input())
    jobs = [tuple(map(int, input().split())) for _ in range(n)]
    best = 0
    for size in range(1, n + 1):
        for chosen in combinations(jobs, size):
            deadlines = sorted(d for d, _ in chosen)
            if all(d >= k + 1 for k, d in enumerate(deadlines)):
                best = max(best, sum(p for _, p in chosen))
    print(best)


main()
