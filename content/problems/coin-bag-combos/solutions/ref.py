def coin_combos(coins: list[int], target: int) -> list[list[int]]:
    coins = sorted(coins)
    out, bag = [], []

    def pick(start, remaining):
        if remaining == 0:
            out.append(list(bag))
            return
        for i in range(start, len(coins)):
            if coins[i] > remaining:
                break
            bag.append(coins[i])
            pick(i, remaining - coins[i])
            bag.pop()

    pick(0, target)
    return out
