def balanced_stretches(results: str) -> int:
    seen = {0: 1}
    score = 0
    count = 0
    for c in results:
        if c == "W":
            score += 1
        elif c == "L":
            score -= 1
        count += seen.get(score, 0)
        seen[score] = seen.get(score, 0) + 1
    return count
