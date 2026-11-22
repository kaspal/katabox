def longest_balanced(results: str) -> int:
    first = {0: -1}
    score = 0
    best = 0
    for i, c in enumerate(results):
        if c == "W":
            score += 1
        elif c == "L":
            score -= 1
        if score in first:
            best = max(best, i - first[score])
        else:
            first[score] = i
    return best
