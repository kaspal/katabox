def validate(inp):
    limit, total = inp["limit"], inp["total"]
    if type(limit) is not int or type(total) is not int:
        return "limit and total are integers"
    if not 1 <= limit <= 10**15:
        return "1 ≤ limit ≤ 10¹⁵"
    if not 1 <= total <= 135:
        return "1 ≤ total ≤ 135"
    return None
