def validate(inp):
    values, target = inp["values"], inp["target"]
    if not isinstance(values, list) or not all(type(v) is int for v in values) or type(target) is not int:
        return "values is an array of integers; target is an integer"
    if not 1 <= len(values) <= 18:
        return "1 ≤ values.length ≤ 18"
    if not all(1 <= v <= 1000 for v in values):
        return "1 ≤ values[i] ≤ 1000"
    if not 1 <= target <= 18_000:
        return "1 ≤ target ≤ 18 000"
    return None
