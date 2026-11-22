import heapq


def assign_rooms(meetings: list[list[int]]) -> list[int]:
    order = sorted(range(len(meetings)), key=lambda i: (meetings[i][0], i))
    free: list[int] = []  # room numbers free now
    busy: list[tuple[int, int]] = []  # (end, room)
    rooms = 0
    out = [0] * len(meetings)
    for i in order:
        s, e = meetings[i]
        while busy and busy[0][0] <= s:
            heapq.heappush(free, heapq.heappop(busy)[1])
        if free:
            room = heapq.heappop(free)
        else:
            rooms += 1
            room = rooms
        out[i] = room
        heapq.heappush(busy, (e, room))
    return out
