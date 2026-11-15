import re

INT = re.compile(r"[0-9]+")


def validate(text):
    lines = text.split("\n")
    if lines and lines[-1] == "":
        lines.pop()
    if len(lines) < 4 or not INT.fullmatch(lines[0]):
        return "n, the species, Q, then the questions"
    n = int(lines[0])
    if not 1 <= n <= 50_000:
        return "1 ≤ n ≤ 5·10⁴"
    ids = lines[1].split()
    if len(ids) != n or not all(INT.fullmatch(p) for p in ids):
        return "the second line holds exactly n species ids"
    if not all(1 <= int(p) <= 1_000_000_000 for p in ids):
        return "1 ≤ id ≤ 10⁹"
    if any(p[0] == "0" for p in ids):
        return "ids have no leading zeros"
    if not INT.fullmatch(lines[2]):
        return "the third line is Q"
    q = int(lines[2])
    if not 1 <= q <= 50_000:
        return "1 ≤ Q ≤ 5·10⁴"
    if len(lines) != 3 + q:
        return "then exactly Q lines l r"
    for line in lines[3:]:
        parts = line.split(" ")
        if len(parts) != 2 or not all(INT.fullmatch(p) for p in parts):
            return "each question is l r"
        l, r = map(int, parts)
        if not 1 <= l <= r <= n:
            return "1 ≤ l ≤ r ≤ n"
    return None
