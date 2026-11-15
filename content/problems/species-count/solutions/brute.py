# Obviously correct: the set of species in every stretch.
def main() -> None:
    n = int(input())
    ids = input().split()
    assert len(ids) == n
    q = int(input())
    for _ in range(q):
        l, r = map(int, input().split())
        print(len(set(ids[l - 1 : r])))


main()
