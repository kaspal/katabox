import re
from datetime import date

NAME = re.compile(r"[a-z]{1,10}")


def validate(inp):
    t = inp["tables"]
    members, visits = t["members"], t["visits"]
    if not 1 <= len(members) <= 300 or len(visits) > 3000:
        return "1 ≤ members ≤ 300, visits ≤ 3000"
    ids = set()
    for mid, name in members:
        if not 1 <= mid <= 10**6 or mid in ids:
            return "member ids are distinct, 1 to 10⁶"
        ids.add(mid)
        if not NAME.fullmatch(name):
            return "a name is 1-10 lowercase letters"
    vids = set()
    for vid, mid, day in visits:
        if not 1 <= vid <= 10**6 or vid in vids:
            return "visit ids are distinct, 1 to 10⁶"
        vids.add(vid)
        if mid not in ids:
            return "every visit belongs to a member"
        try:
            ok = re.fullmatch(r"[0-9]{4}-[0-9]{2}-[0-9]{2}", day) and date.fromisoformat(day).year == 2026
        except ValueError:
            ok = False
        if not ok:
            return "day is a date of 2026"
    return None
