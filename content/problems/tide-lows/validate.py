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
    head = ints(lines[0], 1) if lines else None
    if head is None:
        return "the first line is N"
    n = head[0]
    if not 1 <= n <= 100_000:
        return "1 ≤ N ≤ 10⁵"
    if len(lines) < 3:
        return "N, the levels, then Q"
    levels = ints(lines[1], n)
    if levels is None:
        return "the second line holds exactly N levels"
    if not all(-1_000_000 <= x <= 1_000_000 for x in levels):
        return "-10⁶ ≤ level ≤ 10⁶"
    count = ints(lines[2], 1)
    if count is None:
        return "the third line is Q"
    q = count[0]
    if not 1 <= q <= 50_000:
        return "1 ≤ Q ≤ 5·10⁴"
    if len(lines) != 3 + q:
        return "Q lines l r"
    for i in range(3, 3 + q):
        pair = ints(lines[i], 2)
        if pair is None:
            return "each question is l r"
        l, r = pair
        if not 1 <= l <= r <= n:
            return "1 ≤ l ≤ r ≤ N"
    return None
