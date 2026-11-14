def is_int(s):
    return s.isascii() and s.isdigit()


def validate(text):
    lines = text.split("\n")
    if lines and lines[-1] == "":
        lines.pop()
    if not lines or not is_int(lines[0].strip()):
        return "the first line is T"
    t = int(lines[0])
    if not 1 <= t <= 1000:
        return "1 ≤ T ≤ 1000"
    if len(lines) != 1 + t:
        return "each of the T messages takes one line"
    total = 0
    for idx, line in enumerate(lines[1:], start=1):
        msg = line.strip()
        if not msg.startswith("#") or not is_int(msg[1:]):
            return f"message {idx}: a # followed by digits 0-9"
        n = len(msg) - 1
        if not 1 <= n <= 100_000:
            return "each message has between 1 and 10⁵ digits"
        total += n
    if total > 200_000:
        return "at most 2·10⁵ digits in all the messages together"
    return None
