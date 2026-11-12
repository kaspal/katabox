import re

WORD = re.compile(r"[a-z]{1,20}")


def validate(inp):
    words, prefixes = inp["words"], inp["prefixes"]
    if not isinstance(words, list) or not isinstance(prefixes, list):
        return "words and prefixes are arrays of strings"
    if not 1 <= len(words) <= 10**4:
        return "1 ≤ words.length ≤ 10⁴"
    if not 1 <= len(prefixes) <= 2 * 10**4:
        return "1 ≤ prefixes.length ≤ 2·10⁴"
    if not all(isinstance(s, str) and WORD.fullmatch(s) for s in words + prefixes):
        return "every string has 1 to 20 lowercase English letters"
    return None
