BASE, MOD = 26, 101


def rabin_karp(text, pattern):
    n, m = len(text), len(pattern)
    if m > n:
        return [0, 0]

    def code(ch):
        return ord(ch) - 97

    high = pow(BASE, m - 1, MOD)
    target = window = 0
    for k in range(m):
        target = (target * BASE + code(pattern[k])) % MOD
        window = (window * BASE + code(text[k])) % MOD
    hits, collisions = [], 0
    for i in range(n - m + 1):
        if window == target:
            if text[i:i + m] == pattern:
                hits.append(i)  # @step match
            else:
                collisions += 1  # @step collision
        if i + m < n:
            window = (  # @step roll
                (window - code(text[i]) * high) * BASE + code(text[i + m])
            ) % MOD
    return [len(hits), collisions] + hits
