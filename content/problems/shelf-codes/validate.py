def validate(inp):
    codes, queries = inp["codes"], inp["queries"]
    for arr, name in ((codes, "codes"), (queries, "queries")):
        if not isinstance(arr, list) or not all(type(x) is int for x in arr):
            return name + " is an array of integers"
        if not 1 <= len(arr) <= 10**5:
            return "1 ≤ " + name + ".length ≤ 10⁵"
        if not all(0 <= x <= 10**9 for x in arr):
            return "0 ≤ values ≤ 10⁹"
    if any(codes[i] > codes[i + 1] for i in range(len(codes) - 1)):
        return "codes is sorted in non-decreasing order"
    return None
