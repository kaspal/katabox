import re

NUMBER = re.compile(r"[0-9]+")
TEXT = re.compile(r"[a-z .,;:!?\n]*")
WORD = re.compile(r"[a-z]")


def validate(text):
    first, newline, body = text.partition("\n")
    if not NUMBER.fullmatch(first):
        return "the first line is K"
    k = int(first)
    if not 1 <= k <= 100:
        return "1 ≤ K ≤ 100"
    if not newline:
        return "the text follows the first line"
    if len(body) > 200_000:
        return "the text has at most 200 000 characters"
    if not TEXT.fullmatch(body):
        return "the text holds only lowercase letters, spaces, line breaks and .,;:!?"
    if not WORD.search(body):
        return "the text holds at least one word"
    return None
