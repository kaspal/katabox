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
        return "the first line is N M"
    n, m = head
    if not 1 <= n <= 10_000:
        return "1 ≤ N ≤ 10⁴"
    if not 0 <= m <= 20_000:
        return "0 ≤ M ≤ 2·10⁴"
    if len(lines) < m + 2:
        return "M street lines, then a line with Q"
    for i in range(1, m + 1):
        road = ints(lines[i], 3)
        if road is None:
            return f"street {i}: a b w"
        a, b, w = road
        if not (1 <= a <= n and 1 <= b <= n):
            return "1 ≤ a, b ≤ N"
        if a == b:
            return "a ≠ b"
        if not 1 <= w <= 10_000:
            return "1 ≤ w ≤ 10⁴"
    count = ints(lines[m + 1], 1)
    if count is None:
        return "after the streets, a line with Q"
    q = count[0]
    if not 1 <= q <= 10_000:
        return "1 ≤ Q ≤ 10⁴"
    if len(lines) != m + 2 + q:
        return "Q lines, one junction each"
    for i in range(m + 2, m + 2 + q):
        town = ints(lines[i], 1)
        if town is None:
            return "each question is one junction t"
        if not 1 <= town[0] <= n:
            return "1 ≤ t ≤ N"
    return None
