def validate(inp):
    target, fuel, st = inp["target"], inp["fuel"], inp["stations"]
    if type(target) is not int or type(fuel) is not int or not isinstance(st, list):
        return "target and fuel are integers; stations is a list"
    if not (1 <= target <= 10**9 and 1 <= fuel <= 10**9):
        return "1 ≤ target, fuel ≤ 10⁹"
    if len(st) > 10**5:
        return "0 ≤ stations.length ≤ 10⁵"
    prev = 0
    for s in st:
        if not isinstance(s, list) or len(s) != 2 or not all(type(v) is int for v in s):
            return "each station is [position, litres]"
        if not (prev < s[0] < target):
            return "0 < position < target, positions strictly increasing"
        if not 1 <= s[1] <= 10**9:
            return "1 ≤ litres ≤ 10⁹"
        prev = s[0]
    return None
