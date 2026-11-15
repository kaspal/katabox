import re

NAME = re.compile(r"[A-Z][a-z]{1,10}")


def validate(inp):
    staff = inp["tables"]["staff"]
    if not 1 <= len(staff) <= 500:
        return "1 ≤ staff ≤ 500"
    ids = set()
    for sid, name, _ in staff:
        if not 1 <= sid <= 10**6 or sid in ids:
            return "ids are distinct, 1 to 10⁶"
        ids.add(sid)
        if not NAME.fullmatch(name):
            return "a name is one capitalised word"
    for sid, _, mentor in staff:
        if mentor is not None and (mentor not in ids or mentor == sid):
            return "mentor_id is NULL or another staff member's id"
    return None
