import string


def validate(inp):
    a, b = inp["typed"], inp["meant"]
    if not isinstance(a, str) or not isinstance(b, str):
        return "typed and meant are strings"
    if not 1 <= len(a) <= 1000 or not 1 <= len(b) <= 1000:
        return "1 ≤ typed.length, meant.length ≤ 1000"
    letters = set(string.ascii_lowercase)
    if not set(a) <= letters or not set(b) <= letters:
        return "typed and meant contain only lowercase English letters."
    return None
