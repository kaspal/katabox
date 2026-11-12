def validate(inp):
    lamps = inp["lamps"]
    if not isinstance(lamps, list) or not 1 <= len(lamps) <= 10**5:
        return "1 ≤ lamps.length ≤ 10⁵"
    for l in lamps:
        if not (isinstance(l, list) and len(l) == 2 and all(type(x) is int for x in l)):
            return "lamps[i] = [start, end] with integers"
        if not 0 <= l[0] < l[1] <= 10**9:
            return "0 ≤ start < end ≤ 10⁹"
    return None
