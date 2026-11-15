# Obviously correct: walk up from b through its bosses and look for a.
def main() -> None:
    n = int(input())
    boss = [0, 0] + [int(x) for x in input().split()]
    assert len(boss) == n + 1
    q = int(input())
    for _ in range(q):
        a, b = map(int, input().split())
        above = False
        x = b
        while x != 1:
            x = boss[x]
            if x == a:
                above = True
                break
        print("YES" if above else "NO")


main()
