# Obviously correct: try every minute below the product of the periods (the pattern of
# departures repeats after it, so a meeting, if any, happens before).
def main() -> None:
    t = int(input())
    for _ in range(t):
        n = int(input())
        lines = [tuple(map(int, input().split())) for _ in range(n)]
        limit = 1
        for _, m in lines:
            limit *= m
        answer = -1
        for minute in range(limit):
            if all(minute % m == r for r, m in lines):
                answer = minute
                break
        print(answer)


main()
