import re

LINE = re.compile(r"([1-9][0-9]*) ([1-9][0-9]*)")


def validate(text):
    lines = text.split("\n")
    if lines and lines[-1] == "":
        lines.pop()
    if not lines or not re.fullmatch(r"[1-9][0-9]*", lines[0]):
        return "the first line is n"
    n = int(lines[0])
    if not 1 <= n <= 100_000:
        return "1 ≤ n ≤ 10⁵"
    if len(lines) != 1 + n:
        return "then exactly n job lines"
    for line in lines[1:]:
        m = LINE.fullmatch(line)
        if not m:
            return "each job is deadline fee"
        if not (int(m.group(1)) <= 1_000_000_000 and int(m.group(2)) <= 1_000_000_000):
            return "1 ≤ deadline, fee ≤ 10⁹"
    return None
