def fresh_in_range(plays: list[int], requests: list[list[int]]) -> list[int]:
    # Obviously correct: try every stretch inside each request.
    out = []
    for a, b in requests:
        best = 0
        for i in range(a, b + 1):
            for j in range(i, b + 1):
                part = plays[i : j + 1]
                if len(set(part)) == len(part):
                    best = max(best, len(part))
        out.append(best)
    return out
