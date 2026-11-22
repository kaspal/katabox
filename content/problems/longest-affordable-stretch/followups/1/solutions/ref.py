from bisect import bisect_left


def longest_with_refunds(costs: list[int], budget: int) -> int:
    # A stretch i..j-1 costs prefix[j] - prefix[i]; it fits when prefix[i] >= prefix[j] - budget.
    # Only starts that reach a new high prefix can be the earliest fitting start.
    prefix = 0
    starts: list[int] = [0]  # increasing prefix values
    where: list[int] = [0]  # their positions
    best = 0
    for j, c in enumerate(costs, start=1):
        prefix += c
        k = bisect_left(starts, prefix - budget)
        if k < len(starts):
            best = max(best, j - where[k])
        if prefix > starts[-1]:
            starts.append(prefix)
            where.append(j)
    return best
