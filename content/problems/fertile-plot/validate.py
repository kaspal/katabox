import re

INT = re.compile(r"-?[0-9]+")


def ints(line, count):
    parts = line.split()
    if len(parts) != count or not all(INT.fullmatch(p) for p in parts):
        return None
    return [int(p) for p in parts]


def validate(text):
    lines = text.split("\n")
    if lines and lines[-1] == "":
        lines.pop()
    head = ints(lines[0], 2) if lines else None
    if head is None:
        return "the first line is R C"
    rows, cols = head
    if not (1 <= rows <= 80_000 and 1 <= cols <= 80_000):
        return "1 ≤ R, C ≤ 8·10⁴"
    if rows * cols > 80_000:
        return "R · C ≤ 8·10⁴"
    if len(lines) != 1 + rows:
        return "R lines of scores after the first line"
    for i in range(rows):
        row = ints(lines[1 + i], cols)
        if row is None:
            return f"row {i + 1}: exactly C whole numbers"
        if not all(-1000 <= x <= 1000 for x in row):
            return "-1000 ≤ score ≤ 1000"
    return None
