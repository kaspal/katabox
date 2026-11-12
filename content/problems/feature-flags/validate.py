def validate(inp):
    users, queries = inp["users"], inp["queries"]
    for arr, name in ((users, "users"), (queries, "queries")):
        if not isinstance(arr, list) or not 1 <= len(arr) <= 10**5:
            return "1 ≤ " + name + ".length ≤ 10⁵"
        if not all(type(x) is int and 0 <= x < 1 << 16 for x in arr):
            return "0 ≤ values < 2¹⁶"
    return None
