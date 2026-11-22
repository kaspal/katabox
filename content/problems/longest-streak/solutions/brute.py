def longest_run(values: list[int]) -> int:
    # Obviously correct: from every start, walk while the value stays the same.
    best = 0
    for i in range(len(values)):
        j = i
        while j < len(values) and values[j] == values[i]:
            j += 1
        best = max(best, j - i)
    return best
