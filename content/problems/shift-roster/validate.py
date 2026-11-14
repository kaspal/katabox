import re

NUMBER = re.compile(r"[0-9]+")
ENTRY = re.compile(r"([a-z]+) ([0-9]+)")


def validate(text):
    lines = text.split("\n")
    if lines and lines[-1] == "":
        lines.pop()
    if not lines or not NUMBER.fullmatch(lines[0]):
        return "the first line is N"
    n = int(lines[0])
    if not 1 <= n <= 20_000:
        return "1 ≤ N ≤ 2·10⁴"
    if len(lines) != 1 + n:
        return "N lines follow, one per volunteer"
    seen = set()
    for i, line in enumerate(lines[1:], 2):
        m = ENTRY.fullmatch(line)
        if not m:
            return f"line {i}: a name of lowercase letters, one space, the hours"
        name, hours = m.group(1), int(m.group(2))
        if len(name) > 12:
            return "each name has 1 to 12 lowercase letters"
        if name in seen:
            return "the names are distinct"
        seen.add(name)
        if not 0 <= hours <= 24:
            return "0 ≤ hours ≤ 24"
    return None
