import re


def validate(text):
    lines = text.split("\n")
    if lines and lines[-1] == "":
        lines.pop()
    if len(lines) != 1:
        return "one line: the word"
    if not re.fullmatch(r"[a-z]+", lines[0]):
        return "the word is lowercase letters a-z only"
    if len(lines[0]) > 200_000:
        return "1 ≤ |s| ≤ 2·10⁵"
    return None
