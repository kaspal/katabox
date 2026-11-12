import re


def validate(inp):
    codes = inp["codes"]
    if not isinstance(codes, list) or not all(isinstance(c, str) for c in codes):
        return "codes is an array of strings"
    if not 1 <= len(codes) <= 2 * 10**4:
        return "1 ≤ codes.length ≤ 2·10⁴"
    if not all(re.fullmatch(r"[a-z]{1,12}", c) for c in codes):
        return "1 ≤ codes[i].length ≤ 12, lowercase letters only"
    return None
