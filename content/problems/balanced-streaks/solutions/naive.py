def balanced_stretches(results: str) -> int:
    # Correct but O(n²): extend every run one match at a time.
    n = len(results)
    count = 0
    for i in range(n):
        score = 0
        for j in range(i, n):
            c = results[j]
            if c == "W":
                score += 1
            elif c == "L":
                score -= 1
            if score == 0:
                count += 1
    return count
