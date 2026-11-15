import re

NAME = re.compile(r"[a-z]{1,12}")
CITY = re.compile(r"[A-Z][a-z]{1,11}")
EMAIL = re.compile(r"[a-z@. ]{0,30}")


def validate(inp):
    readers = inp["tables"]["readers"]
    if not 1 <= len(readers) <= 300:
        return "1 ≤ readers ≤ 300"
    ids = set()
    for rid, name, email, city in readers:
        if not 1 <= rid <= 10**6 or rid in ids:
            return "ids are distinct, 1 to 10⁶"
        ids.add(rid)
        if not NAME.fullmatch(name):
            return "a name is 1-12 lowercase letters"
        if not CITY.fullmatch(city):
            return "a city is one capitalised word"
        if email is not None and not EMAIL.fullmatch(email):
            return "an email is NULL or up to 30 characters of a-z, @, . and spaces"
    return None
