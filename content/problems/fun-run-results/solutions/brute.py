def finish_order(names: list[str], times: list[int]) -> list[str]:
    # Obviously correct: repeatedly take the runner that beats everyone left.
    left = list(range(len(names)))
    board = []
    while left:
        best = left[0]
        for i in left:
            if times[i] < times[best] or (times[i] == times[best] and names[i] < names[best]):
                best = i
        board.append(names[best])
        left.remove(best)
    return board
