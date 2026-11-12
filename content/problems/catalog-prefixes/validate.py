import re

WORD = re.compile(r"[a-z]{1,20}")


def validate(inp):
    titles, typed = inp["titles"], inp["typed"]
    if not isinstance(titles, list) or not isinstance(typed, list):
        return "titles and typed are arrays of strings"
    if not 1 <= len(titles) <= 10**4:
        return "1 ≤ titles.length ≤ 10⁴"
    if not 1 <= len(typed) <= 2 * 10**4:
        return "1 ≤ typed.length ≤ 2·10⁴"
    for s in titles + typed:
        if not isinstance(s, str) or not WORD.fullmatch(s):
            return "every string has 1 to 20 lowercase English letters"
    return None
