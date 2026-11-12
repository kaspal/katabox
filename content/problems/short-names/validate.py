import re

NAME = re.compile(r"[a-z]{1,20}")


def validate(inp):
    names = inp["names"]
    if not isinstance(names, list):
        return "names is an array of strings"
    if not 1 <= len(names) <= 2 * 10**4:
        return "1 ≤ names.length ≤ 2·10⁴"
    if not all(isinstance(s, str) and NAME.fullmatch(s) for s in names):
        return "names have 1 to 20 lowercase English letters"
    if len(set(names)) != len(names):
        return "all names are different"
    return None
