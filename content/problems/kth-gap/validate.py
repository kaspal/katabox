def validate(inp):
    p, k = inp["posts"], inp["k"]
    if not isinstance(p, list) or not 2 <= len(p) <= 5 * 10**4:
        return "2 ≤ posts.length ≤ 5·10⁴"
    if not all(type(v) is int and 0 <= v <= 10**6 for v in p):
        return "0 ≤ posts[i] ≤ 10⁶"
    n = len(p)
    if type(k) is not int or not 1 <= k <= n * (n - 1) // 2:
        return "1 ≤ k ≤ n · (n − 1) / 2"
    return None
