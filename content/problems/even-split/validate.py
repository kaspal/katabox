def validate(inp):
    paid = inp["paid"]
    if not isinstance(paid, list) or not all(type(x) in (int, float) for x in paid):
        return "paid is an array of numbers"
    if not 1 <= len(paid) <= 2 * 10**4:
        return "1 ≤ paid.length ≤ 2·10⁴"
    if not all(0 <= x <= 10**4 for x in paid):
        return "0 ≤ paid[i] ≤ 10⁴"
    return None
