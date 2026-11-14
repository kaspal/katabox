def subsets(values):
    out, chosen = [], []

    def grow(start):
        out.append(list(chosen))  # @step record
        for k in range(start, len(values)):
            chosen.append(values[k])  # @step choose
            grow(k + 1)
            chosen.pop()  # @step undo

    grow(0)
    flat = [len(out)]
    for subset in out:
        flat += [len(subset)] + subset
    return flat
