import re

NAME = re.compile(r"[a-z]{1,12}( [a-z]{1,12})?")


def validate(inp):
    products = inp["tables"]["products"]
    if not 1 <= len(products) <= 500:
        return "1 ≤ products ≤ 500"
    ids = set()
    for pid, name, price, stock in products:
        if not 1 <= pid <= 10**6 or pid in ids:
            return "ids are distinct, 1 to 10⁶"
        ids.add(pid)
        if not NAME.fullmatch(name):
            return "a name is one or two lowercase words"
        if not 1 <= price <= 10**6:
            return "1 ≤ price_cents ≤ 10⁶"
        if stock not in (0, 1):
            return "in_stock is 0 or 1"
    return None
