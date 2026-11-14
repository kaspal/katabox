def lis(values):
    n = len(values)
    length = [1] * n
    prev = [-1] * n
    for i in range(n):
        for j in range(i):
            if values[j] < values[i] and length[j] + 1 > length[i]:
                length[i] = length[j] + 1  # @step extend
                prev[i] = j
    end = max(range(n), key=lambda k: (length[k], -k))
    chain = []
    while end != -1:
        chain.append(values[end])  # @step trace
        end = prev[end]
    return [len(chain)] + chain[::-1]
