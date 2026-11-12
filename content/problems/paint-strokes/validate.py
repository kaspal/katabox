def validate(inp):
    s = inp["strokes"]
    if not isinstance(s, list):
        return "strokes is an array of pairs"
    if not 1 <= len(s) <= 10**5:
        return "1 ≤ strokes.length ≤ 10⁵"
    for x in s:
        if not (isinstance(x, list) and len(x) == 2 and all(type(v) is int for v in x)
                and 0 <= x[0] < x[1] <= 10**9):
            return "strokes[i] = [start, end] with 0 ≤ start < end ≤ 10⁹"
    return None
