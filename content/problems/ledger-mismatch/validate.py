import re

REF = re.compile(r"P[0-9]{4}")


def validate(inp):
    t = inp["tables"]
    if len(t["bank"]) > 2000 or len(t["books"]) > 2000 or not (t["bank"] or t["books"]):
        return "at most 2000 rows per table, and at least one row in all"
    for table in ("bank", "books"):
        ids, refs = set(), set()
        for rid, ref, amount in t[table]:
            if not 1 <= rid <= 10**6 or rid in ids:
                return f"{table} ids are distinct, 1 to 10⁶"
            ids.add(rid)
            if not REF.fullmatch(ref) or ref in refs:
                return f"{table} refs are distinct, P and four digits"
            refs.add(ref)
            if not -10**7 <= amount <= 10**7:
                return "|amount_cents| ≤ 10⁷"
    return None
