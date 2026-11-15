"""Products in the band, in plain Python."""


def solve(tables):
    rows = [(-price, name, pid) for pid, name, price, stock in tables["products"]
            if stock == 1 and 2000 <= price <= 4999]
    rows.sort()
    return {"columns": ["name", "price_cents"], "rows": [[name, -neg] for neg, name, _ in rows]}
