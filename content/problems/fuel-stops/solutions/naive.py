def fuel_stops(target: int, fuel: int, stations: list[list[int]]) -> int:
    # Correct but O(n²): far[t] = the farthest reach with t stops, updated per station.
    far = [fuel] + [0] * len(stations)
    for i, (position, litres) in enumerate(stations):
        for t in range(i, -1, -1):
            if far[t] >= position and far[t] + litres > far[t + 1]:
                far[t + 1] = far[t] + litres
    for t, reach in enumerate(far):
        if reach >= target:
            return t
    return -1
