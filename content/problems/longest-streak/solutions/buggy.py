def longest_run(values: list[int]) -> int:
    best = 0
    run = 1
    for i in range(1, len(values)):
        if values[i] == values[i - 1]:
            run += 1
        else:
            best = max(best, run)
            run = 1
    return best
