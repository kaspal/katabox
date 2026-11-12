def clubs_left(n: int, merges: list[list[int]]) -> list[int]:
    # Correct but O(n·m): a club label per player, and every merge relabels by
    # scanning all players.
    label = list(range(n))
    clubs = n
    out = []
    for a, b in merges:
        la, lb = label[a], label[b]
        if la != lb:
            for i in range(n):
                if label[i] == lb:
                    label[i] = la
            clubs -= 1
        out.append(clubs)
    return out
