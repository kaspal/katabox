def min_rooms(meetings: list[list[int]]) -> int:
    # Correct but O(n²): for every meeting, count the meetings running when it starts.
    best = 0
    for s, _ in meetings:
        running = 0
        for a, b in meetings:
            if a <= s < b:
                running += 1
        if running > best:
            best = running
    return best
