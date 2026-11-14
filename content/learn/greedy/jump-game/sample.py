def can_jump(jumps):
    reach = 0
    for i, jump in enumerate(jumps):
        if i > reach:
            return [0, reach]  # @step stuck
        if i + jump > reach:
            reach = i + jump  # @step farther
    return [1, min(reach, len(jumps) - 1)]
