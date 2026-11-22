def f(n):
    memo = {}
    def ways(k):
        if k <= 1:
            return 1
        if k in memo:
            return memo[k]
        memo[k] = (ways(k - 1) + ways(k - 2)) % 1_000_000_007
        return memo[k]
    return ways(n)
