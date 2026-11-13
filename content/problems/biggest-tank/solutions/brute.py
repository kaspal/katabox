def biggest_tank(boards: list[int]) -> int:
    # Obviously correct: every pair of boards.
    n = len(boards)
    return max((j - i) * min(boards[i], boards[j]) for i in range(n) for j in range(i + 1, n))
