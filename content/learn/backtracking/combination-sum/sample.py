def combination_sum(target, candidates):
    candidates = sorted(candidates)
    out, chosen = [], []

    def search(start, left):
        if left == 0:
            out.append(list(chosen))  # @step found
            return
        for k in range(start, len(candidates)):
            if candidates[k] > left:
                break  # @step prune
            chosen.append(candidates[k])  # @step choose
            search(k, left - candidates[k])
            chosen.pop()

    search(0, target)
    flat = [len(out)]
    for combo in out:
        flat += [len(combo)] + combo
    return flat
