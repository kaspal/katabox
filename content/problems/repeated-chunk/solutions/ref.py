def longest_repeat(text: str) -> int:
    n = len(text)
    M, B = (1 << 61) - 1, 131
    h, pw = [0] * (n + 1), [1] * (n + 1)
    for i, ch in enumerate(text):
        h[i + 1] = (h[i] * B + ord(ch)) % M
        pw[i + 1] = pw[i] * B % M

    def repeats(length):
        seen = set()
        p = pw[length]
        for i in range(n - length + 1):
            x = (h[i + length] - h[i] * p) % M
            if x in seen:
                return True
            seen.add(x)
        return False

    lo, hi = 0, n - 1
    while lo < hi:
        mid = (lo + hi + 1) // 2
        if repeats(mid):
            lo = mid
        else:
            hi = mid - 1
    return lo
