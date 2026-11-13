def biggest_tank(boards: list[int]) -> int:
    # Correct but O(n²): every pair of boards.
    best = 0
    for i, a in enumerate(boards):
        for j in range(i + 1, len(boards)):
            area = (j - i) * min(a, boards[j])
            if area > best:
                best = area
    return best
