from itertools import combinations


def cheapest_network(n: int, cables: list[list[int]]) -> int:
    # Obviously correct: try every set of n − 1 cables and keep the cheapest one that
    # connects all villages.
    if n == 1:
        return 0

    def connects(chosen):
        reach, frontier = {0}, [0]
        while frontier:
            v = frontier.pop()
            for a, b, _ in chosen:
                for x, y in ((a, b), (b, a)):
                    if x == v and y not in reach:
                        reach.add(y)
                        frontier.append(y)
        return len(reach) == n

    best = -1
    for chosen in combinations(cables, n - 1):
        cost = sum(c for _, _, c in chosen)
        if (best < 0 or cost < best) and connects(chosen):
            best = cost
    return best
