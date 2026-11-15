import re

CITY = re.compile(r" {0,2}[A-Za-z]{2,12} {0,2}")


def validate(inp):
    flights = inp["tables"]["flights"]
    if not 1 <= len(flights) <= 2000:
        return "1 ≤ flights ≤ 2000"
    ids = set()
    for fid, origin, dest in flights:
        if not 1 <= fid <= 10**6 or fid in ids:
            return "ids are distinct, 1 to 10⁶"
        ids.add(fid)
        if not CITY.fullmatch(origin) or not CITY.fullmatch(dest):
            return "a city is 2-12 letters, with at most two spaces on each side"
    return None
