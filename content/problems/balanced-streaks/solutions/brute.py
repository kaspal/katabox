def balanced_stretches(results: str) -> int:
    # Obviously correct: check every run.
    n = len(results)
    return sum(
        1
        for i in range(n)
        for j in range(i + 1, n + 1)
        if results[i:j].count("W") == results[i:j].count("L")
    )
