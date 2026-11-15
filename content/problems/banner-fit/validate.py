import re


def validate(text):
    lines = text.split("\n")
    if lines and lines[-1] == "":
        lines.pop()
    if not lines or not re.fullmatch(r"[0-9]+ [0-9]+", lines[0]):
        return "the first line is R C"
    r, c = map(int, lines[0].split())
    if not (1 <= r <= 1500 and 1 <= c <= 1500):
        return "1 ≤ R, C ≤ 1500"
    if len(lines) != 1 + r:
        return "then exactly R rows"
    for row in lines[1:]:
        if len(row) != c or not re.fullmatch(r"[.#]+", row):
            return "each row has exactly C characters, . or #"
    return None
