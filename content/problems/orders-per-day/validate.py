from datetime import datetime


def validate(inp):
    orders = inp["tables"]["orders"]
    if len(orders) > 3000:
        return "at most 3000 orders"
    ids = set()
    for oid, placed, total in orders:
        if not 1 <= oid <= 10**6 or oid in ids:
            return "ids are distinct, 1 to 10⁶"
        ids.add(oid)
        try:
            ok = len(placed) == 16 and datetime.strptime(placed, "%Y-%m-%d %H:%M").year == 2026
        except ValueError:
            ok = False
        if not ok:
            return "placed is YYYY-MM-DD HH:MM in 2026"
        if not 1 <= total <= 10**6:
            return "1 ≤ total_cents ≤ 10⁶"
    return None
