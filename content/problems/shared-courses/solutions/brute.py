"""Pairs of students who share a course, in plain Python."""


def solve(tables):
    names = {sid: name for sid, name in tables["students"]}
    by_course = {}
    for _, sid, course in tables["enrollments"]:
        by_course.setdefault(course, set()).add(sid)
    pairs = set()
    for ids in by_course.values():
        for a in ids:
            for b in ids:
                if a < b:
                    pairs.add((a, b))
    return {"columns": ["student_a", "student_b"],
            "rows": [[names[a], names[b]] for a, b in sorted(pairs)]}
