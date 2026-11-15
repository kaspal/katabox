import re

TITLE = re.compile(r"[a-z]{1,10}( [a-z]{1,10}){0,2}")


def validate(inp):
    listings = inp["tables"]["listings"]
    if not 1 <= len(listings) <= 1000:
        return "1 ≤ listings ≤ 1000"
    ids = set()
    for lid, title, rating, hidden in listings:
        if not 1 <= lid <= 10**6 or lid in ids:
            return "ids are distinct, 1 to 10⁶"
        ids.add(lid)
        if not TITLE.fullmatch(title):
            return "a title is 1-3 lowercase words"
        if not 0 <= rating <= 5:
            return "0 ≤ rating ≤ 5"
        if hidden not in (0, 1):
            return "hidden is 0 or 1"
    return None
