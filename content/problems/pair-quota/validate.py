import re

INT = re.compile(r"-?[0-9]+")


def ints(line, count=None):
    parts = line.split()
    if not all(INT.fullmatch(p) for p in parts) or (count is not None and len(parts) != count):
        return None
    return [int(p) for p in parts]


def validate(text):
    lines = text.split("\n")
    if lines and lines[-1] == "":
        lines.pop()
    head = ints(lines[0], 1) if lines else None
    if head is None:
        return "the first line is T"
    t = head[0]
    if not 1 <= t <= 10_000:
        return "1 ≤ T ≤ 10⁴"
    if len(lines) != 1 + 2 * t:
        return "each case takes two lines: n K, then the n values"
    total = 0
    for c in range(t):
        nk = ints(lines[1 + 2 * c], 2)
        if nk is None:
            return f"case {c + 1}: the line n K"
        n, k = nk
        if n < 1:
            return "1 ≤ n"
        if not -2_000_000_000 <= k <= 2_000_000_000:
            return "|K| ≤ 2·10⁹"
        values = ints(lines[2 + 2 * c], n)
        if values is None:
            return f"case {c + 1}: exactly n values"
        if not all(-1_000_000_000 <= v <= 1_000_000_000 for v in values):
            return "|value| ≤ 10⁹"
        total += n
    if total > 100_000:
        return "Σn ≤ 10⁵"
    return None
