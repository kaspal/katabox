import re

INT = re.compile(r"[0-9]+")


def validate(text):
    lines = text.split("\n")
    if lines and lines[-1] == "":
        lines.pop()
    if len(lines) < 3:
        return "n Q, the prices, then Q budgets"
    head = lines[0].split()
    if len(head) != 2 or not all(INT.fullmatch(p) for p in head):
        return "the first line is n Q"
    n, q = map(int, head)
    if not (1 <= n <= 50_000 and 1 <= q <= 50_000):
        return "1 ≤ n, Q ≤ 5·10⁴"
    prices = lines[1].split()
    if len(prices) != n or not all(INT.fullmatch(p) for p in prices):
        return "the second line holds exactly n prices"
    if not all(1 <= int(p) <= 1_000_000_000 for p in prices):
        return "1 ≤ price ≤ 10⁹"
    if len(lines) != 2 + q:
        return "then exactly Q budget lines"
    for line in lines[2:]:
        if not INT.fullmatch(line):
            return "each budget is a whole number on its own line"
        if not 0 <= int(line) <= 50_000_000_000_000:
            return "0 ≤ budget ≤ 5·10¹³"
    return None
