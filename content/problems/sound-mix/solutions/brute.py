# Obviously correct: every stretch, its set of sources.
def main() -> None:
    n, k = map(int, input().split())
    ids = input().split()
    assert len(ids) == n
    print(sum(1 for i in range(n) for j in range(i, n) if len(set(ids[i : j + 1])) == k))


main()
