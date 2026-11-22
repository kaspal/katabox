def longest_balanced(results: str) -> int:
    # Obviously correct: every run, counted directly.
    best = 0
    for i in range(len(results)):
        for j in range(i, len(results)):
            run = results[i : j + 1]
            if run.count("W") == run.count("L"):
                best = max(best, j - i + 1)
    return best
