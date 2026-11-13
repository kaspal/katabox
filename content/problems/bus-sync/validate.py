import math
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
        return "the first line is T"
    t = head[0]
    if not 1 <= t <= 2000:
        return "1 ≤ T ≤ 2000"
    i = 1
    for case in range(t):
        count = ints(lines[i], 1) if i < len(lines) else None
        if count is None:
            return f"timetable {case + 1}: a line with n, then n lines r m"
        n = count[0]
        if not 1 <= n <= 6:
            return "1 ≤ n ≤ 6"
        if i + 1 + n > len(lines):
            return f"timetable {case + 1}: n lines r m"
        total = 1
        for k in range(n):
            pair = ints(lines[i + 1 + k], 2)
            if pair is None:
                return f"timetable {case + 1}: each line is r m"
            r, m = pair
            if not 1 <= m <= 1_000_000:
                return "1 ≤ m ≤ 10⁶"
            if not 0 <= r < m:
                return "0 ≤ r < m"
            total = total // math.gcd(total, m) * m
        if total > 9 * 10**15:
            return "the lcm of each timetable's m is at most 9·10¹⁵"
        i += 1 + n
    if i != len(lines):
        return "exactly T timetables"
    return None
