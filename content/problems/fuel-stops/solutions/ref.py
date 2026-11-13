import heapq


def fuel_stops(target: int, fuel: int, stations: list[list[int]]) -> int:
    reach, stops, passed = fuel, 0, []
    for position, litres in stations + [[target, 0]]:
        while reach < position:
            if not passed:
                return -1
            reach -= heapq.heappop(passed)
            stops += 1
        heapq.heappush(passed, -litres)
    return stops
