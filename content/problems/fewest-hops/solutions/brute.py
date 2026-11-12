def fewest_hops(n: int, links: list[list[int]]) -> int:
    # Obviously correct: grow the set of stations reachable in k hops, k = 0, 1, 2, ...
    reach = {0}
    for k in range(n):
        if n - 1 in reach:
            return k
        reach = reach | {b for a, b in links if a in reach} | {a for a, b in links if b in reach}
    return -1
