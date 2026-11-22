"""rotated-shelf--f1 constraints."""


def validate(inp):
    shelf, queries = inp["shelf"], inp["queries"]
    if not isinstance(shelf, list) or not 1 <= len(shelf) <= 10**5:
        return "1 ≤ shelf.length ≤ 10⁵"
    if not all(type(x) is int and 1 <= x <= 10**9 for x in shelf):
        return "1 ≤ shelf[i] ≤ 10⁹"
    n = len(shelf)
    drops = sum(1 for i in range(n - 1) if shelf[i] >= shelf[i + 1])
    if drops > 1 or (drops == 1 and shelf[-1] >= shelf[0]) or len(set(shelf)) != n:
        return "different numbers: a sorted list with a block moved from the start to the end"
    if not isinstance(queries, list) or not 1 <= len(queries) <= 10**5:
        return "1 ≤ queries.length ≤ 10⁵"
    if not all(type(k) is int and 1 <= k <= 10**9 for k in queries):
        return "1 ≤ queries[j] ≤ 10⁹"
    return None
