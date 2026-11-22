def race_order(names: list[str], times: list[int]) -> list[str]:
    # Obviously correct: repeatedly take the fastest remaining runner (smaller name on ties).
    left = list(zip(times, names))
    out = []
    while left:
        best = min(left)
        left.remove(best)
        out.append(best[1])
    return out
