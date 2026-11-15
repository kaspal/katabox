import re

STATION = re.compile(r"[A-Z][a-z]{2,11}")


def validate(inp):
    trips = inp["tables"]["trips"]
    if len(trips) > 3000:
        return "at most 3000 trips"
    ids = set()
    for tid, a, b, minutes in trips:
        if not 1 <= tid <= 10**6 or tid in ids:
            return "ids are distinct, 1 to 10⁶"
        ids.add(tid)
        if not STATION.fullmatch(a) or not STATION.fullmatch(b):
            return "a station is one capitalised word"
        if not 1 <= minutes <= 600:
            return "1 ≤ minutes ≤ 600"
    return None
