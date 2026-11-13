def trip_totals(miles: list[int], trips: list[list[int]]) -> list[int]:
    # Correct but O(n·q): walk every trip's days.
    out = []
    for a, b in trips:
        total = 0
        for i in range(a, b + 1):
            total += miles[i]
        out.append(total)
    return out
