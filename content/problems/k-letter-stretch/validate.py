"""k-letter-stretch constraints."""


def validate(inp):
    t, k = inp["text"], inp["k"]
    if not isinstance(t, str) or not 1 <= len(t) <= 10**5 or not all("a" <= c <= "z" for c in t):
        return "text has 1 to 10⁵ lowercase English letters"
    if type(k) is not int or not 1 <= k <= 26:
        return "1 ≤ k ≤ 26"
    return None
