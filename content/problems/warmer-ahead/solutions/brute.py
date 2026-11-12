def days_until_warmer(temps: list[int]) -> list[int]:
    # Obviously correct: for each day, the first later warmer day.
    out = []
    for i, t in enumerate(temps):
        later = [j for j in range(i + 1, len(temps)) if temps[j] > t]
        out.append(later[0] - i if later else 0)
    return out
