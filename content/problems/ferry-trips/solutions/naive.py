def min_trips(weights: list[int], limit: int) -> int:
    # Correct but O(n²): heaviest first, each looks through everyone left for the
    # heaviest companion that still fits.
    w = sorted(weights, reverse=True)
    n = len(w)
    used = [False] * n
    trips = 0
    for i in range(n):
        if used[i]:
            continue
        used[i] = True
        for j in range(i + 1, n):
            if not used[j] and w[i] + w[j] <= limit:
                used[j] = True
                break
        trips += 1
    return trips
