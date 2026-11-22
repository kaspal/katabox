def best_run(profits: list[int], k: int) -> int:
    # Obviously correct: every run of 1 to k consecutive days.
    return max(sum(profits[i : i + length]) for length in range(1, k + 1) for i in range(len(profits) - length + 1))
