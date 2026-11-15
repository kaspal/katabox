# Obviously correct: try every pair of positions.
from itertools import combinations


def main() -> None:
    t = int(input())
    for _ in range(t):
        n, k = map(int, input().split())
        values = list(map(int, input().split()))
        assert len(values) == n
        print(sum(1 for a, b in combinations(values, 2) if a + b <= k))


main()
