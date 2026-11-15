import re

INT = re.compile(r"[0-9]+")


def validate(text):
    lines = text.split("\n")
    if lines and lines[-1] == "":
        lines.pop()
    if len(lines) != 2:
        return "two lines: n K, then the n source ids"
    head = lines[0].split()
    if len(head) != 2 or not all(INT.fullmatch(p) for p in head):
        return "the first line is n K"
    n, k = map(int, head)
    if not 1 <= k <= n <= 100_000:
        return "1 ≤ K ≤ n ≤ 10⁵"
    ids = lines[1].split()
    if len(ids) != n or not all(INT.fullmatch(p) for p in ids):
        return "the second line holds exactly n ids"
    if not all(1 <= int(p) <= 1_000_000_000 for p in ids):
        return "1 ≤ id ≤ 10⁹"
    if any(p[0] == "0" for p in ids):
        return "ids have no leading zeros"
    return None
