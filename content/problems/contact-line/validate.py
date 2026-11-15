import re

NAME = re.compile(r"[A-Z][a-z]{1,10}")
PHONE = re.compile(r"[0-9]{3}-[0-9]{4}")
EMAIL = re.compile(r"[a-z]{1,10}@[a-z]{1,10}\.test")


def validate(inp):
    clients = inp["tables"]["clients"]
    if not 1 <= len(clients) <= 300:
        return "1 ≤ clients ≤ 300"
    ids = set()
    for cid, first, last, phone, email, prefers in clients:
        if not 1 <= cid <= 10**6 or cid in ids:
            return "ids are distinct, 1 to 10⁶"
        ids.add(cid)
        if not NAME.fullmatch(first) or not NAME.fullmatch(last):
            return "names are one capitalised word"
        if phone is not None and not PHONE.fullmatch(phone):
            return "a phone is NULL or ddd-dddd"
        if email is not None and not EMAIL.fullmatch(email):
            return "an email is NULL or name@domain.test"
        if prefers not in ("phone", "email", None):
            return "prefers is 'phone', 'email' or NULL"
    return None
