def validate(text):
    lines = text.split("\n")
    if lines and lines[-1] == "":
        lines.pop()
    if not lines or not lines[0].strip().isdigit():
        return "the first line is T"
    t = int(lines[0])
    if not 1 <= t <= 1000:
        return "1 ≤ T ≤ 1000"
    if len(lines) != 1 + 2 * t:
        return "each day takes two lines: n, then the n prices"
    total = 0
    for d in range(t):
        n_line, prices_line = lines[1 + 2 * d], lines[2 + 2 * d]
        if not n_line.strip().isdigit():
            return f"day {d + 1}: n is a whole number"
        n = int(n_line)
        if not 0 <= n <= 1000:
            return "0 ≤ n ≤ 1000"
        prices = prices_line.split()
        if len(prices) != n or not all(p.isdigit() for p in prices):
            return f"day {d + 1}: the price line holds exactly n whole numbers"
        if not all(1 <= int(p) <= 10_000 for p in prices):
            return "1 ≤ price ≤ 10 000"
        total += n
    if total > 50_000:
        return "at most 50 000 prices in the whole input"
    return None
