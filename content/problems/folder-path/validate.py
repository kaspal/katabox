import re


def validate(inp):
    p = inp["path"]
    if not isinstance(p, str) or not 1 <= len(p) <= 3000:
        return "1 ≤ path.length ≤ 3000"
    if not re.fullmatch(r"/[A-Za-z0-9._/]*", p):
        return "path starts with / and has only letters, digits, ., _ and /"
    return None
