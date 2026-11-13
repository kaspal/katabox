import re

SIZE = re.compile(r"([0-9]+) ([0-9]+)")
NUMBER = re.compile(r"[0-9]+")
ROW = re.compile(r"[#.]+")
QUESTION = re.compile(r"(row|col) ([0-9]+)")


def validate(text):
    lines = text.split("\n")
    if lines and lines[-1] == "":
        lines.pop()
    m = SIZE.fullmatch(lines[0]) if lines else None
    if not m:
        return "the first line is R C"
    r, c = int(m.group(1)), int(m.group(2))
    if not (1 <= r <= 300 and 1 <= c <= 300):
        return "1 ≤ R, C ≤ 300"
    if len(lines) < 2 + r:
        return "R map lines, then a line with Q"
    for i in range(r):
        row = lines[1 + i]
        if len(row) != c or not ROW.fullmatch(row):
            return f"map row {i}: exactly C characters, each # or ."
    if not NUMBER.fullmatch(lines[1 + r]):
        return "after the map, a line with Q"
    q = int(lines[1 + r])
    if not 1 <= q <= 20_000:
        return "1 ≤ Q ≤ 2·10⁴"
    if len(lines) != 2 + r + q:
        return "Q question lines follow"
    for line in lines[2 + r :]:
        m = QUESTION.fullmatch(line)
        if not m:
            return "each question is row r or col c"
        index = int(m.group(2))
        if m.group(1) == "row" and not index < r:
            return "0 ≤ r < R"
        if m.group(1) == "col" and not index < c:
            return "0 ≤ c < C"
    return None
