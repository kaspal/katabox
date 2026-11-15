import re

PAIR = re.compile(r"([1-9][0-9]*) ([1-9][0-9]*)")


def validate(text):
    lines = text.split("\n")
    if lines and lines[-1] == "":
        lines.pop()
    if not lines or not re.fullmatch(r"[1-9][0-9]* [1-9][0-9]* [1-9][0-9]*", lines[0]):
        return "the first line is R C Q"
    r, c, q = map(int, lines[0].split())
    if not (r <= 1000 and c <= 1000):
        return "1 ≤ R, C ≤ 1000"
    if not q <= 50_000:
        return "1 ≤ Q ≤ 5·10⁴"
    if len(lines) != 1 + q:
        return "then exactly Q lines r c"
    for line in lines[1:]:
        m = PAIR.fullmatch(line)
        if not m:
            return "each square is r c"
        if not (int(m.group(1)) <= r and int(m.group(2)) <= c):
            return "1 ≤ r ≤ R and 1 ≤ c ≤ C"
    return None
