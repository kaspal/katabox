import re

REGION = re.compile(r"[A-Z][a-z]{2,11}")


def validate(inp):
    sales = inp["tables"]["sales"]
    if not 1 <= len(sales) <= 2000:
        return "1 ≤ sales ≤ 2000"
    ids = set()
    for sid, region, amount in sales:
        if not 1 <= sid <= 10**6 or sid in ids:
            return "ids are distinct, 1 to 10⁶"
        ids.add(sid)
        if not REGION.fullmatch(region):
            return "a region is one capitalised word"
        if not 1 <= amount <= 10**7:
            return "1 ≤ amount_cents ≤ 10⁷"
    return None
