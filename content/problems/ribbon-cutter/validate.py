def validate(inp):
    ribbons, pieces = inp["ribbons"], inp["pieces"]
    if not isinstance(ribbons, list) or not all(type(x) is int for x in ribbons) or type(pieces) is not int:
        return "ribbons is an array of integers; pieces is an integer"
    if not 1 <= len(ribbons) <= 10**5:
        return "1 ≤ ribbons.length ≤ 10⁵"
    if not all(1 <= r <= 10**7 for r in ribbons):
        return "1 ≤ ribbons[i] ≤ 10⁷"
    if not 1 <= pieces <= 10**9:
        return "1 ≤ pieces ≤ 10⁹"
    return None
