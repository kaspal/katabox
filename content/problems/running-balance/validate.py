import re
from datetime import date

ACCOUNT = re.compile(r"AC[0-9]{3}")


def validate(inp):
    rows = inp["tables"]["transactions"]
    if len(rows) > 3000:
        return "at most 3000 transactions"
    ids = set()
    for tid, acct, day, amount in rows:
        if not 1 <= tid <= 10**6 or tid in ids:
            return "ids are distinct, 1 to 10⁶"
        ids.add(tid)
        if not ACCOUNT.fullmatch(acct):
            return "an account is AC and three digits"
        try:
            ok = re.fullmatch(r"[0-9]{4}-[0-9]{2}-[0-9]{2}", day) and date.fromisoformat(day).year == 2026
        except ValueError:
            ok = False
        if not ok:
            return "day is a date of 2026"
        if not -10**6 <= amount <= 10**6 or amount == 0:
            return "amount_cents is non-zero, within ±10⁶"
    return None
