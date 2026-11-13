def trip_totals(miles: list[int], trips: list[list[int]]) -> list[int]:
    prefix = [0]
    for m in miles:
        prefix.append(prefix[-1] + m)
    return [prefix[b + 1] - prefix[a] for a, b in trips]
