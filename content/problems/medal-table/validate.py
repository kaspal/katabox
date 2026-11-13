import re


def validate(inp):
    c, m = inp["countries"], inp["medals"]
    if not isinstance(c, list) or not isinstance(m, list) or not 1 <= len(c) == len(m) <= 10**4:
        return "1 ≤ countries.length = medals.length ≤ 10⁴"
    if not all(isinstance(s, str) and re.fullmatch(r"[a-z]{1,20}", s) for s in c) or len(set(c)) != len(c):
        return "names are 1-20 lowercase letters, all different"
    for x in m:
        if not isinstance(x, list) or len(x) != 3 or not all(type(v) is int and 0 <= v <= 1000 for v in x):
            return "medals[i] = [gold, silver, bronze], each 0..1000"
    return None
