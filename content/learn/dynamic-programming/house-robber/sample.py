def rob(values):
    n = len(values)
    best = [0] * (n + 1)
    best[1] = values[0]
    for i in range(2, n + 1):
        skip = best[i - 1]
        take = best[i - 2] + values[i - 1]
        best[i] = max(skip, take)  # @step choose
    chosen, i = [], n
    while i > 0:
        if best[i] == best[i - 1]:
            i -= 1
        else:
            chosen.append(i - 1)  # @step trace
            i -= 2
    return [best[n]] + chosen[::-1]
