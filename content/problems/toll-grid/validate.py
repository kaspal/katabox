def validate(inp):
    tolls = inp["tolls"]
    if not isinstance(tolls, list) or not 1 <= len(tolls) <= 150:
        return "1 ≤ rows ≤ 150"
    cols = len(tolls[0]) if isinstance(tolls[0], list) else -1
    if not 1 <= cols <= 150:
        return "1 ≤ cols ≤ 150"
    for row in tolls:
        if not isinstance(row, list) or len(row) != cols:
            return "all rows of tolls have the same length"
        if not all(type(x) is int and 1 <= x <= 10**4 for x in row):
            return "1 ≤ tolls[r][c] ≤ 10⁴"
    return None
