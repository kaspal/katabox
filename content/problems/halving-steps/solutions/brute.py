def halving_steps(n: int) -> int:
    # Obviously correct: record the whole path, then count its moves.
    path = [n]
    while path[-1] != 1:
        x = path[-1]
        path.append(x // 2 if x % 2 == 0 else 3 * x + 1)
    return len(path) - 1
