import re

NAME = re.compile(r"[A-Z][a-z]{1,11}")


def validate(inp):
    staff = inp["tables"]["staff"]
    if not 1 <= len(staff) <= 1000:
        return "1 ≤ staff ≤ 1000"
    boss = {}
    for sid, name, b in staff:
        if not 1 <= sid <= 10**6 or sid in boss:
            return "ids are distinct, 1 to 10⁶"
        if not NAME.fullmatch(name):
            return "a name is one capitalised word"
        boss[sid] = b
    if sum(1 for b in boss.values() if b is None) != 1:
        return "exactly one director (boss_id NULL)"
    for sid, b in boss.items():
        if b is not None and b not in boss:
            return "boss_id is NULL or another staff member's id"
    # No loops: everyone reaches the director.
    for sid in boss:
        seen, at = set(), sid
        while at is not None:
            if at in seen:
                return "the reporting lines have no loops"
            seen.add(at)
            at = boss[at]
    return None
