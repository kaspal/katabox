def longest_run(values: list[int]) -> int:
    best = run = 1
    for i in range(1, len(values)):
        run = run + 1 if values[i] == values[i - 1] else 1
        best = max(best, run)
    return best
