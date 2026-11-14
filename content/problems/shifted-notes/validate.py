import re

NUMBER = re.compile(r"[0-9]+")
TEXT = re.compile(r"[ -~\n]*")


def validate(text):
    first, newline, body = text.partition("\n")
    if not NUMBER.fullmatch(first):
        return "the first line is s"
    if not 0 <= int(first) <= 25:
        return "0 ≤ s ≤ 25"
    if not newline or not body:
        return "there is at least one text line"
    if len(body) > 150_000:
        return "the text has at most 150 000 characters"
    if not TEXT.fullmatch(body):
        return "each character is printable ASCII or a line break"
    return None
