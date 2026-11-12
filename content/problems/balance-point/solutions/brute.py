def balance_point(weights: list[int]) -> int:
    # Obviously correct: add up both sides for every index.
    for i in range(len(weights)):
        if sum(weights[:i]) == sum(weights[i + 1:]):
            return i
    return -1
