"""Big orders with their customers, in plain Python."""


def solve(tables):
    customers = {cid: (name, city) for cid, name, city in tables["customers"]}
    rows = []
    for oid, cid, _placed, total in tables["orders"]:
        if total >= 10000:
            name, city = customers[cid]
            rows.append((-total, oid, name, city))
    rows.sort()
    return {"columns": ["order_id", "name", "city", "total_cents"],
            "rows": [[oid, name, city, -neg] for neg, oid, name, city in rows]}
