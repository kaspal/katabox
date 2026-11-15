import re


def validate(text):
    lines = text.split("\n")
    if lines and lines[-1] == "":
        lines.pop()
    if not lines or not re.fullmatch(r"[0-9]+ [0-9]+ [0-9]+", lines[0]):
        return "the first line is R C K"
    r, c, k = map(int, lines[0].split())
    if not (1 <= r <= 40 and 1 <= c <= 40):
        return "1 ≤ R, C ≤ 40"
    if not 1 <= k <= 8:
        return "1 ≤ K ≤ 8"
    if len(lines) != 1 + r:
        return "then exactly R picture rows"
    for row in lines[1:]:
        if not re.fullmatch(r"[.#*o+]+", row) or len(row) != c:
            return "each row has exactly C characters from . # * o +"
    return None
