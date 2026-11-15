import re

LINE = re.compile(r"(0|[1-9][0-9]*) ([1-9][0-9]*) ([1-9][0-9]*)")


def validate(text):
    lines = text.split("\n")
    if lines and lines[-1] == "":
        lines.pop()
    if not lines or not re.fullmatch(r"[1-9][0-9]*", lines[0]):
        return "the first line is n"
    n = int(lines[0])
    if not 1 <= n <= 50_000:
        return "1 ≤ n ≤ 5·10⁴"
    if len(lines) != 1 + n:
        return "then exactly n job lines"
    for line in lines[1:]:
        m = LINE.fullmatch(line)
        if not m:
            return "each job is arrival priority pages"
        a, p, g = int(m.group(1)), int(m.group(2)), int(m.group(3))
        if not 0 <= a <= 1_000_000_000:
            return "0 ≤ arrival ≤ 10⁹"
        if not 1 <= p <= 1_000_000_000:
            return "1 ≤ priority ≤ 10⁹"
        if not 1 <= g <= 10_000:
            return "1 ≤ pages ≤ 10⁴"
    return None
