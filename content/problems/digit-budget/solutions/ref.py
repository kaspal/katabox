def digit_budget(limit: int, total: int) -> int:
    digits = [int(c) for c in str(limit)]
    n = len(digits)
    memo = {}

    def free(pos, s):
        # Numbers for the remaining positions with any digits (not tight).
        if s > total:
            return 0
        if pos == n:
            return 1 if s == total else 0
        key = (pos, s)
        if key not in memo:
            memo[key] = sum(free(pos + 1, s + x) for x in range(10))
        return memo[key]

    count, s = 0, 0
    for pos, top in enumerate(digits):
        # Put a smaller digit here: the rest is free.
        for x in range(top):
            count += free(pos + 1, s + x)
        s += top
    return count + (1 if s == total else 0)
