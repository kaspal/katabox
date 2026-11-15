import re

ENTRY = re.compile(r"([a-z]{1,10}):([1-9][0-9]*)")


def validate(text):
    lines = text.split("\n")
    if lines and lines[-1] == "":
        lines.pop()
    if not lines or not re.fullmatch(r"[1-9][0-9]*", lines[0]):
        return "the first line is N"
    n = int(lines[0])
    if not 1 <= n <= 1000:
        return "1 ≤ N ≤ 1000"
    if len(lines) != 1 + n:
        return "then exactly N label lines"
    for line in lines[1:]:
        entries = line.split(";")
        if not 1 <= len(entries) <= 20:
            return "a label has 1 to 20 entries separated by ;"
        for e in entries:
            m = ENTRY.fullmatch(e)
            if not m:
                return "each entry is name:quantity (name 1-10 lowercase letters, no spaces)"
            if not 1 <= int(m.group(2)) <= 1000:
                return "1 ≤ quantity ≤ 1000"
    return None
