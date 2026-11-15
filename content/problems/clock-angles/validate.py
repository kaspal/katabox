import re

TIME = re.compile(r"([0-9]{2}):([0-9]{2})")


def validate(text):
    lines = text.split("\n")
    if lines and lines[-1] == "":
        lines.pop()
    if not lines or not re.fullmatch(r"[0-9]+", lines[0]):
        return "the first line is Q"
    q = int(lines[0])
    if not 1 <= q <= 1440:
        return "1 ≤ Q ≤ 1440"
    if len(lines) != 1 + q:
        return "Q lines hh:mm"
    for line in lines[1:]:
        m = TIME.fullmatch(line)
        if not m:
            return "each time is hh:mm with two digits each"
        if not (0 <= int(m.group(1)) <= 23 and 0 <= int(m.group(2)) <= 59):
            return "00 ≤ hh ≤ 23 and 00 ≤ mm ≤ 59"
    return None
