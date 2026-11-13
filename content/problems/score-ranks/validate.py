def is_int(s):
    return s.isascii() and s.isdigit()


def validate(text):
    lines = text.split("\n")
    if lines and lines[-1] == "":
        lines.pop()
    if len(lines) != 4:
        return "the input has four lines: N, the scores, Q, the asked scores"
    if not is_int(lines[0].strip()):
        return "the first line is N"
    n = int(lines[0])
    if not 1 <= n <= 20_000:
        return "1 ≤ N ≤ 20 000"
    scores = lines[1].split()
    if len(scores) != n or not all(is_int(s) for s in scores):
        return "the second line holds exactly N scores"
    if not all(0 <= int(s) <= 10**9 for s in scores):
        return "0 ≤ score ≤ 10⁹"
    if not is_int(lines[2].strip()):
        return "the third line is Q"
    q = int(lines[2])
    if not 1 <= q <= 20_000:
        return "1 ≤ Q ≤ 20 000"
    asked = lines[3].split()
    if len(asked) != q or not all(is_int(x) for x in asked):
        return "the fourth line holds exactly Q asked scores"
    if not all(0 <= int(x) <= 10**9 for x in asked):
        return "0 ≤ x ≤ 10⁹"
    return None
