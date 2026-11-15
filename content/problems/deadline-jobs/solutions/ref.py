import sys


def main() -> None:
    data = sys.stdin.buffer.read().split()
    n = int(data[0])
    jobs = sorted(((int(data[2 + 2 * i]), int(data[1 + 2 * i])) for i in range(n)), reverse=True)
    # Best fee first, each on the latest free day by its deadline (only days 1..n matter).
    # free[d] points towards the latest free day ≤ d; day 0 means "none left". Taking a
    # day links it to the day before (union-find with path halving).
    free = list(range(n + 1))

    def latest(d: int) -> int:
        while free[d] != d:
            free[d] = free[free[d]]
            d = free[d]
        return d

    total = 0
    for fee, deadline in jobs:
        day = latest(min(deadline, n))
        if day > 0:
            free[day] = day - 1
            total += fee
    print(total)


main()
