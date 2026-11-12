def fewest_coins(coins: list[int], amount: int) -> int:
    # Obviously correct: breadth-first search over the amounts paid so far, one token
    # per step; the first time `amount` is reached uses the fewest tokens.
    frontier, seen, steps = {0}, {0}, 0
    while frontier:
        if amount in frontier:
            return steps
        nxt = set()
        for x in frontier:
            for c in coins:
                if x + c <= amount and x + c not in seen:
                    seen.add(x + c)
                    nxt.add(x + c)
        frontier, steps = nxt, steps + 1
    return -1
