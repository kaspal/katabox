from itertools import combinations


def fuel_stops(target: int, fuel: int, stations: list[list[int]]) -> int:
    # Obviously correct (few stations): try every set of stops, fewest first.
    for k in range(len(stations) + 1):
        for chosen in combinations(stations, k):
            tank, pos, ok = fuel, 0, True
            for p, litres in sorted(chosen) + [[target, 0]]:
                tank -= p - pos
                pos = p
                if tank < 0:
                    ok = False
                    break
                tank += litres
            if ok:
                return k
    return -1
