def assign_rooms(meetings: list[list[int]]) -> list[int]:
    # Obviously correct: in order of start (ties by position), give each meeting the
    # smallest room whose earlier meetings have all ended by its start.
    order = sorted(range(len(meetings)), key=lambda i: (meetings[i][0], i))
    out = [0] * len(meetings)
    for i in order:
        s = meetings[i][0]
        room = 1
        while any(out[j] == room and meetings[j][1] > s for j in range(len(meetings)) if out[j]):
            room += 1
        out[i] = room
    return out
