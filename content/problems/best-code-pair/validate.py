def validate(inp):
    codes = inp["codes"]
    if not isinstance(codes, list) or not all(type(v) is int for v in codes):
        return "codes is a list of integers"
    if not 2 <= len(codes) <= 10**5:
        return "2 ≤ codes.length ≤ 10⁵"
    if not all(0 <= v < 2**20 for v in codes):
        return "0 ≤ codes[i] < 2²⁰ (that is, below 1 048 576)"
    return None
