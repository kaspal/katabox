def min_rooms(meetings: list[list[int]]) -> int:
    starts = sorted(m[0] for m in meetings)
    ends = sorted(m[1] for m in meetings)
    rooms = best = 0
    j = 0
    for s in starts:
        while ends[j] <= s:
            rooms -= 1
            j += 1
        rooms += 1
        best = max(best, rooms)
    return best
