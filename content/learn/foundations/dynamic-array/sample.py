def append_all(values):
    capacity = 1
    data = [None] * capacity
    size = 0
    copies = 0
    for v in values:
        if size == capacity:  # @step check
            capacity *= 2
            bigger = [None] * capacity
            for i in range(size):
                bigger[i] = data[i]  # @step copy
                copies += 1
            data = bigger
        data[size] = v  # @step write
        size += 1
    return [capacity, copies] + data[:size]
