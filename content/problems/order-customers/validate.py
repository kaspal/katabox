import re
from datetime import date

NAME = re.compile(r"[A-Z][a-z]{1,10}")


def validate(inp):
    t = inp["tables"]
    customers, orders = t["customers"], t["orders"]
    if not 1 <= len(customers) <= 200 or len(orders) > 2000:
        return "1 ≤ customers ≤ 200, orders ≤ 2000"
    ids = set()
    for cid, name, city in customers:
        if not 1 <= cid <= 10**6 or cid in ids:
            return "customer ids are distinct, 1 to 10⁶"
        ids.add(cid)
        if not NAME.fullmatch(name) or not NAME.fullmatch(city):
            return "names and cities are one capitalised word"
    oids = set()
    for oid, cid, placed, total in orders:
        if not 1 <= oid <= 10**6 or oid in oids:
            return "order ids are distinct, 1 to 10⁶"
        oids.add(oid)
        if cid not in ids:
            return "every order has a customer"
        try:
            if date.fromisoformat(placed).year != 2026 or not re.fullmatch(r"[0-9]{4}-[0-9]{2}-[0-9]{2}", placed):
                return "placed is a date of 2026"
        except ValueError:
            return "placed is a date of 2026"
        if not 1 <= total <= 10**7:
            return "1 ≤ total_cents ≤ 10⁷"
    return None
