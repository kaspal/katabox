import re
from datetime import date

NAME = re.compile(r"[a-z][a-z0-9_]{2,11}")


def validate(inp):
    logins = inp["tables"]["logins"]
    if len(logins) > 3000:
        return "at most 3000 logins"
    ids = set()
    for lid, user, day in logins:
        if not 1 <= lid <= 10**6 or lid in ids:
            return "ids are distinct, 1 to 10⁶"
        ids.add(lid)
        if not NAME.fullmatch(user):
            return "a username is 3-12 of a-z, 0-9 and _, starting with a letter"
        try:
            ok = re.fullmatch(r"[0-9]{4}-[0-9]{2}-[0-9]{2}", day) and 2025 <= date.fromisoformat(day).year <= 2026
        except ValueError:
            ok = False
        if not ok:
            return "day is a date of 2025 or 2026"
    return None
