def kadane(values):
    best, best_start, best_end = values[0], 0, 0
    current, start = 0, 0
    for i, value in enumerate(values):
        if current <= 0:
            current, start = value, i  # @step restart
        else:
            current += value  # @step extend
        if current > best:
            best, best_start, best_end = current, start, i  # @step record
    return [best, best_start, best_end]
