import re
from datetime import date


def validate(inp):
    sales = inp["tables"]["sales"]
    if len(sales) > 3000:
        return "at most 3000 sales"
    ids = set()
    for sid, day, amount in sales:
        if not 1 <= sid <= 10**6 or sid in ids:
            return "ids are distinct, 1 to 10⁶"
        ids.add(sid)
        try:
            ok = re.fullmatch(r"[0-9]{4}-[0-9]{2}-[0-9]{2}", day) and 2020 <= date.fromisoformat(day).year <= 2026
        except ValueError:
            ok = False
        if not ok:
            return "sold_on is a date from 2020 to 2026"
        if not 1 <= amount <= 10**6:
            return "1 ≤ amount_cents ≤ 10⁶"
    return None
