def is_int(s):
    return s.isascii() and s.isdigit()


def validate(text):
    lines = text.split("\n")
    if lines and lines[-1] == "":
        lines.pop()
    if len(lines) < 3 or not is_int(lines[0].strip()):
        return "the first line is N"
    n = int(lines[0])
    if not 1 <= n <= 30_000:
        return "1 ≤ N ≤ 30 000"
    ids = lines[1].split()
    if len(ids) != n or not all(is_int(x) for x in ids):
        return "the second line holds exactly N badge ids"
    if not all(1 <= int(x) <= 10**9 for x in ids):
        return "1 ≤ badge id ≤ 10⁹"
    if not is_int(lines[2].strip()):
        return "the third line is Q"
    q = int(lines[2])
    if not 1 <= q <= 30_000:
        return "1 ≤ Q ≤ 30 000"
    if len(lines) != 3 + q:
        return "each of the Q questions takes one line"
    for line in lines[3:]:
        p = line.strip()
        if not is_int(p):
            return "each question line holds one number p"
        if not 1 <= int(p) <= n:
            return "1 ≤ p ≤ N"
    return None
