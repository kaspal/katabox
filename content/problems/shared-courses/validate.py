import re

NAME = re.compile(r"[a-z]{1,10}")
COURSE = re.compile(r"[A-Z]{3}[0-9]{3}")


def validate(inp):
    t = inp["tables"]
    students, enrollments = t["students"], t["enrollments"]
    if not 1 <= len(students) <= 150 or len(enrollments) > 600:
        return "1 ≤ students ≤ 150, enrollments ≤ 600"
    ids, names = set(), set()
    for sid, name in students:
        if not 1 <= sid <= 10**6 or sid in ids:
            return "student ids are distinct, 1 to 10⁶"
        ids.add(sid)
        if not NAME.fullmatch(name) or name in names:
            return "names are distinct, 1-10 lowercase letters"
        names.add(name)
    eids, seen = set(), set()
    for eid, sid, course in enrollments:
        if not 1 <= eid <= 10**6 or eid in eids:
            return "enrollment ids are distinct, 1 to 10⁶"
        eids.add(eid)
        if sid not in ids:
            return "every enrollment belongs to a student"
        if not COURSE.fullmatch(course):
            return "a course code is three capitals and three digits"
        if (sid, course) in seen:
            return "a student enrolls in a course at most once"
        seen.add((sid, course))
    return None
