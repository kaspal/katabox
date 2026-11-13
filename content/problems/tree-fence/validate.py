def validate(inp):
    trees = inp["trees"]
    if not isinstance(trees, list) or not 1 <= len(trees) <= 10**5:
        return "1 ≤ trees.length ≤ 10⁵"
    for t in trees:
        if not isinstance(t, list) or len(t) != 2 or not all(type(v) is int and abs(v) <= 10**7 for v in t):
            return "All coordinates are integers with absolute value at most 10⁷."
    return None
