# Obviously correct: read line by line, exactly as the input is laid out.
def main() -> None:
    t = int(input())
    for _ in range(t):
        n = int(input())
        prices = [int(x) for x in input().split()]
        assert len(prices) == n
        print(len(prices), sum(prices), max(prices) if prices else 0)


main()
