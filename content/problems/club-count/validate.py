"""club-count constraints."""


def validate(inp):
    n, fs = inp["n"], inp["friendships"]
    if type(n) is not int or not 2 <= n <= 10**4:
        return "2 ≤ n ≤ 10⁴"
    if not isinstance(fs, list) or not 1 <= len(fs) <= 2 * 10**4:
        return "1 ≤ friendships.length ≤ 2·10⁴"
    for f in fs:
        if not isinstance(f, list) or len(f) != 2 or not all(type(x) is int and 0 <= x < n for x in f):
            return "each friendship is [a, b] with 0 ≤ a, b < n"
        if f[0] == f[1]:
            return "a friendship joins two different people"
    return None
