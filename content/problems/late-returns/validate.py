import re
from datetime import date

NAME = re.compile(r"[a-z]{1,10}")
TITLE = re.compile(r"[A-Z][a-z]{0,14}( [A-Z][a-z]{0,14}){0,2}")
DAY = re.compile(r"2026-[0-9]{2}-[0-9]{2}")


def is_day(s):
    if not isinstance(s, str) or not DAY.fullmatch(s):
        return False
    try:
        date.fromisoformat(s)
    except ValueError:
        return False
    return True


def validate(inp):
    tables = inp["tables"]
    members, loans = tables["members"], tables["loans"]
    if not 1 <= len(members) <= 200:
        return "1 ≤ members ≤ 200"
    if len(loans) > 2000:
        return "at most 2000 loans"
    ids = set()
    for mid, name in members:
        if not 1 <= mid <= 10**6 or mid in ids:
            return "member ids are distinct, 1 to 10⁶"
        if not NAME.fullmatch(name):
            return "a name is 1-10 lowercase letters"
        ids.add(mid)
    loan_ids = set()
    for lid, member_id, title, due, returned in loans:
        if not 1 <= lid <= 10**6 or lid in loan_ids:
            return "loan ids are distinct, 1 to 10⁶"
        loan_ids.add(lid)
        if member_id not in ids:
            return "every loan belongs to a member"
        if not TITLE.fullmatch(title):
            return "a title is 1-3 capitalised words"
        if not is_day(due):
            return "due is a date of 2026 (YYYY-MM-DD)"
        if returned is not None and not is_day(returned):
            return "returned is a date of 2026 (YYYY-MM-DD) or NULL"
    return None
