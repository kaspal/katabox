"""typo-distance--f1 constraints."""


def validate(inp):
    t, m, k = inp["typed"], inp["meant"], inp["k"]
    if not isinstance(t, str) or not isinstance(m, str) or type(k) is not int:
        return "typed and meant are strings; k is an integer"
    if not 1 <= len(t) <= 5 * 10**4 or not 1 <= len(m) <= 5 * 10**4:
        return "1 ≤ typed.length, meant.length ≤ 5·10⁴"
    if not all("a" <= c <= "z" for c in t + m):
        return "only lowercase English letters"
    if not 0 <= k <= 20:
        return "0 ≤ k ≤ 20"
    return None
