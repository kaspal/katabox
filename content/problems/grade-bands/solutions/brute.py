def grade_bands(scores: list[int]) -> list[str]:
    # Obviously correct: look each score up in a table of every possible score.
    table = {}
    for s in range(101):
        table[s] = "A" if 90 <= s <= 100 else "B" if 80 <= s <= 89 else "C" if 70 <= s <= 79 else "D" if 60 <= s <= 69 else "F"
    return [table[s] for s in scores]
