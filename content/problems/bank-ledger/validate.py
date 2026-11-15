import re

NAME = re.compile(r"[a-z]{1,10}")
INT = re.compile(r"-?(0|[1-9][0-9]*)")


def validate(text):
    lines = text.split("\n")
    if lines and lines[-1] == "":
        lines.pop()
    if not lines or not re.fullmatch(r"[1-9][0-9]*", lines[0]):
        return "the first line is Q"
    q = int(lines[0])
    if not 1 <= q <= 100_000:
        return "1 ≤ Q ≤ 10⁵"
    if len(lines) != 1 + q:
        return "then exactly Q commands"
    for line in lines[1:]:
        parts = line.split(" ")
        cmd = parts[0]
        if cmd in ("BEGIN", "ROLLBACK", "COMMIT"):
            if len(parts) != 1:
                return f"{cmd} takes nothing"
        elif cmd == "GET":
            if len(parts) != 2 or not NAME.fullmatch(parts[1]):
                return "GET name (1-10 lowercase letters)"
        elif cmd in ("SET", "ADD"):
            if len(parts) != 3 or not NAME.fullmatch(parts[1]) or not INT.fullmatch(parts[2]):
                return f"{cmd} name amount"
            x = int(parts[2])
            if cmd == "SET" and not 0 <= x <= 1_000_000_000:
                return "SET: 0 ≤ amount ≤ 10⁹"
            if cmd == "ADD" and not -1_000_000_000 <= x <= 1_000_000_000:
                return "ADD: -10⁹ ≤ amount ≤ 10⁹"
        elif cmd == "COUNT":
            if len(parts) != 2 or not INT.fullmatch(parts[1]):
                return "COUNT amount"
            if abs(int(parts[1])) > 10**14:
                return "COUNT: |amount| ≤ 10¹⁴"
        else:
            return "commands are SET, ADD, GET, COUNT, BEGIN, ROLLBACK, COMMIT"
    return None
