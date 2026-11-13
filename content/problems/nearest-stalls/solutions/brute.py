def nearest_stalls(stalls: list[list[int]], k: int) -> list[int]:
    # Obviously correct: sort every stall by (squared distance, index).
    order = sorted(range(len(stalls)), key=lambda i: (stalls[i][0] ** 2 + stalls[i][1] ** 2, i))
    return order[:k]
