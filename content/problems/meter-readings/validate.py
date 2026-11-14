import re

NUMBER = re.compile(r"[0-9]+")


def validate(text):
    lines = text.split("\n")
    if lines and lines[-1] == "":
        lines.pop()
    if not lines or not NUMBER.fullmatch(lines[0]):
        return "the first line is N"
    n = int(lines[0])
    if not 1 <= n <= 60_000:
        return "1 ≤ N ≤ 6·10⁴"
    if len(lines) != 1 + n:
        return "N lines follow, one reading each"
    for line in lines[1:]:
        if not NUMBER.fullmatch(line):
            return "each reading is a whole number on its own line"
        if not 0 <= int(line) <= 1_000_000:
            return "0 ≤ reading ≤ 10⁶"
    return None
