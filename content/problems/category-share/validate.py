import re

CATEGORY = re.compile(r"[a-z]{3,12}")


def validate(inp):
    sales = inp["tables"]["sales"]
    if not 1 <= len(sales) <= 3000:
        return "1 ≤ sales ≤ 3000"
    ids = set()
    for sid, category, amount in sales:
        if not 1 <= sid <= 10**6 or sid in ids:
            return "ids are distinct, 1 to 10⁶"
        ids.add(sid)
        if not CATEGORY.fullmatch(category):
            return "a category is 3-12 lowercase letters"
        if not 1 <= amount <= 10**6:
            return "1 ≤ amount_cents ≤ 10⁶"
    return None
