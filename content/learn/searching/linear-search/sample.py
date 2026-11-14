def linear_search(a, target):
    for i in range(len(a)):
        if a[i] == target:  # @step check
            return i  # @step found
    return -1  # @step missing
