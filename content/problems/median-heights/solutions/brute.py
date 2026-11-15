# Obviously correct: the median is the height with at most half of the others strictly
# below it and at most half strictly above it (the lower one when n is even).
def main() -> None:
    t = int(input())
    for _ in range(t):
        n = int(input())
        heights = list(map(int, input().split()))
        assert len(heights) == n
        rank = (n - 1) // 2  # how many singers stand before the median in height order
        for h in sorted(set(heights)):
            below = sum(1 for x in heights if x < h)
            same = sum(1 for x in heights if x == h)
            if below <= rank < below + same:
                print(h)
                break


main()
