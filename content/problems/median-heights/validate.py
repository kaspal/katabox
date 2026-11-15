import re

INT = re.compile(r"[1-9][0-9]*")


def validate(text):
    lines = text.split("\n")
    if lines and lines[-1] == "":
        lines.pop()
    if not lines or not INT.fullmatch(lines[0]):
        return "the first line is T"
    t = int(lines[0])
    if not 1 <= t <= 1000:
        return "1 ≤ T ≤ 1000"
    if len(lines) != 1 + 2 * t:
        return "each section takes two lines: n, then the n heights"
    total = 0
    for c in range(t):
        if not INT.fullmatch(lines[1 + 2 * c]):
            return f"section {c + 1}: n is a whole number ≥ 1"
        n = int(lines[1 + 2 * c])
        heights = lines[2 + 2 * c].split(" ")
        if len(heights) != n or not all(INT.fullmatch(h) for h in heights):
            return f"section {c + 1}: exactly n heights"
        if not all(int(h) <= 1_000_000_000 for h in heights):
            return "1 ≤ height ≤ 10⁹"
        total += n
    if total > 100_000:
        return "Σn ≤ 10⁵"
    return None
