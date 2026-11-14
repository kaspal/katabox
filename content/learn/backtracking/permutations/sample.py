def permutations(values):
    n = len(values)
    used, chosen, out = [False] * n, [], []

    def place():
        if len(chosen) == n:
            out.extend(chosen)  # @step record
            return
        for k in range(n):
            if not used[k]:
                used[k] = True
                chosen.append(values[k])  # @step choose
                place()
                chosen.pop()  # @step undo
                used[k] = False

    place()
    return [len(out) // n] + out
