import re
from datetime import datetime

CODE = re.compile(r"[A-Z]{3}")
CITY = re.compile(r"[A-Z][a-z]{1,11}")
FLIGHT = re.compile(r"KB[0-9]{3}")
NAME = re.compile(r"[a-z]{1,10}")


def validate(inp):
    t = inp["tables"]
    airports, flights, bookings = t["airports"], t["flights"], t["bookings"]
    if not 2 <= len(airports) <= 30 or not 1 <= len(flights) <= 300 or len(bookings) > 2000:
        return "2 ≤ airports ≤ 30, 1 ≤ flights ≤ 300, bookings ≤ 2000"
    codes = set()
    for code, city in airports:
        if not CODE.fullmatch(code) or code in codes or not CITY.fullmatch(city):
            return "airport codes are distinct, three capitals; a city is one capitalised word"
        codes.add(code)
    fids, departs = set(), set()
    for fid, no, src, dst, dep in flights:
        if not 1 <= fid <= 10**6 or fid in fids:
            return "flight ids are distinct, 1 to 10⁶"
        fids.add(fid)
        if not FLIGHT.fullmatch(no):
            return "a flight number is KB and three digits"
        if src not in codes or dst not in codes or src == dst:
            return "a flight goes between two different airports"
        try:
            ok = len(dep) == 16 and datetime.strptime(dep, "%Y-%m-%d %H:%M").year == 2026
        except ValueError:
            ok = False
        if not ok or dep in departs:
            return "departure times are distinct, YYYY-MM-DD HH:MM in 2026"
        departs.add(dep)
    bids = set()
    for bid, passenger, fid in bookings:
        if not 1 <= bid <= 10**6 or bid in bids:
            return "booking ids are distinct, 1 to 10⁶"
        bids.add(bid)
        if not NAME.fullmatch(passenger) or fid not in fids:
            return "a booking has a passenger (1-10 lowercase letters) and an existing flight"
    return None
