# Obviously correct: for every k, is the total of the k cheapest rations within the budget?
def main() -> None:
    n, q = map(int, input().split())
    prices = sorted(map(int, input().split()))
    assert len(prices) == n
    for _ in range(q):
        budget = int(input())
        print(max(k for k in range(n + 1) if sum(prices[:k]) <= budget))


main()
