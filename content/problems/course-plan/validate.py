import re

PAIR = re.compile(r"([0-9]+) ([0-9]+)")


def validate(text):
    lines = text.split("\n")
    if lines and lines[-1] == "":
        lines.pop()
    head = PAIR.fullmatch(lines[0]) if lines else None
    if not head:
        return "the first line is n m"
    n, m = int(head.group(1)), int(head.group(2))
    if not 1 <= n <= 30_000:
        return "1 ≤ n ≤ 3·10⁴"
    if not 0 <= m <= 40_000:
        return "0 ≤ m ≤ 4·10⁴"
    if len(lines) != 1 + m:
        return "then exactly m lines a b"
    for line in lines[1:]:
        pair = PAIR.fullmatch(line)
        if not pair:
            return "each rule is a b"
        a, b = int(pair.group(1)), int(pair.group(2))
        if not (1 <= a <= n and 1 <= b <= n):
            return "1 ≤ a, b ≤ n"
        if a == b:
            return "a ≠ b"
    return None
