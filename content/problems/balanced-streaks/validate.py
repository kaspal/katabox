import re


def validate(inp):
    results = inp["results"]
    if not isinstance(results, str) or not re.fullmatch(r"[WLD]{1,100000}", results):
        return "1 ≤ results.length ≤ 10⁵, only W, L and D"
    return None
