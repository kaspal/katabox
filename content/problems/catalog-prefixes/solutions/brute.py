def prefix_counts(titles: list[str], typed: list[str]) -> list[int]:
    # Obviously correct: compare every typed text with every title.
    return [sum(1 for t in titles if t[: len(p)] == p) for p in typed]
