def pair_with_sum(a, target):
    i, j = 0, len(a) - 1
    while i < j:
        total = a[i] + a[j]  # @step sum
        if total == target:
            return [i, j]  # @step found
        if total < target:  # @step move
            i += 1
        else:
            j -= 1
    return [-1, -1]  # @step missing
