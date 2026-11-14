# Obviously correct: for each question, count the scans among the first p whose id
# does not appear earlier (each member's first visit), reading line by line.
def main() -> None:
    n = int(input())
    scans = [int(x) for x in input().split()]
    assert len(scans) == n
    q = int(input())
    for _ in range(q):
        p = int(input())
        firsts = 0
        for i in range(p):
            if scans[i] not in scans[:i]:
                firsts += 1
        print(firsts)


main()
