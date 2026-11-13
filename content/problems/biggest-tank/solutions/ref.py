def biggest_tank(boards: list[int]) -> int:
    i, j, best = 0, len(boards) - 1, 0
    while i < j:
        if boards[i] <= boards[j]:
            best = max(best, (j - i) * boards[i])
            i += 1
        else:
            best = max(best, (j - i) * boards[j])
            j -= 1
    return best
