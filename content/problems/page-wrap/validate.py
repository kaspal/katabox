import re

WORD = re.compile(r"[a-z]{1,20}")


def validate(inp):
    words, width = inp["words"], inp["width"]
    if not isinstance(words, list) or not 1 <= len(words) <= 10**4:
        return "1 ≤ words.length ≤ 10⁴"
    if not all(isinstance(w, str) and WORD.fullmatch(w) for w in words):
        return "words have 1 to 20 lowercase English letters"
    if type(width) is not int or not 1 <= width <= 100:
        return "1 ≤ width ≤ 100"
    return None
