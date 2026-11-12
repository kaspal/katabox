def validate(inp):
    ice = inp["ice"]
    if not isinstance(ice, list) or not all(isinstance(r, str) for r in ice):
        return "ice is an array of strings"
    if not 1 <= len(ice) <= 400:
        return "1 ≤ ice.length ≤ 400"
    w = len(ice[0])
    if not 1 <= w <= 400 or any(len(r) != w for r in ice):
        return "1 ≤ ice[i].length ≤ 400, all rows the same length"
    if any(ch not in "#." for r in ice for ch in r):
        return "every character is # or ."
    return None
