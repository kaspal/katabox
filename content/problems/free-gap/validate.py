import re

TIME = re.compile(r"([0-9]{2}):([0-9]{2})")


def minutes(t):
    m = TIME.fullmatch(t)
    if not m or int(m.group(1)) > 23 or int(m.group(2)) > 59:
        return None
    return int(m.group(1)) * 60 + int(m.group(2))


def validate(text):
    lines = text.split("\n")
    if lines and lines[-1] == "":
        lines.pop()
    if not lines or not re.fullmatch(r"[0-9]+", lines[0]):
        return "the first line is T"
    t = int(lines[0])
    if not 1 <= t <= 100:
        return "1 ≤ T ≤ 100"
    pos = 1
    total = 0
    for d in range(t):
        if pos >= len(lines):
            return f"day {d + 1} is missing"
        head = lines[pos].split(" ")
        if len(head) != 3 or not re.fullmatch(r"[0-9]+", head[2]):
            return "each day starts with open close k"
        open_, close, k = minutes(head[0]), minutes(head[1]), int(head[2])
        if open_ is None or close is None:
            return "times are hh:mm, 00:00 to 23:59"
        if not open_ < close:
            return "open < close"
        if not 0 <= k <= 1000:
            return "0 ≤ k ≤ 1000"
        if pos + 1 + k > len(lines):
            return "then exactly k slot lines"
        for line in lines[pos + 1 : pos + 1 + k]:
            parts = line.split(" ")
            if len(parts) != 2:
                return "each slot is start end"
            s, e = minutes(parts[0]), minutes(parts[1])
            if s is None or e is None:
                return "times are hh:mm, 00:00 to 23:59"
            if not open_ <= s < e <= close:
                return "open ≤ start < end ≤ close"
        total += k
        pos += 1 + k
    if pos != len(lines):
        return "nothing after the last day"
    if total > 20_000:
        return "Σk ≤ 2·10⁴"
    return None
