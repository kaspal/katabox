def validate(inp):
    boss, pairs = inp["boss"], inp["pairs"]
    if not isinstance(boss, list) or not all(type(b) is int for b in boss):
        return "boss is an array of integers"
    n = len(boss)
    if not 1 <= n <= 5 * 10**4:
        return "1 ≤ n ≤ 5·10⁴"
    if sum(1 for b in boss if b == -1) != 1:
        return "exactly one boss[i] is -1"
    if not all(b == -1 or 0 <= b < n for b in boss):
        return "every other boss[i] is between 0 and n − 1"
    state = [0] * n  # 0 unknown, 1 on the current path, 2 reaches the president
    for i in range(n):
        path = []
        b = i
        while b != -1 and state[b] == 0:
            state[b] = 1
            path.append(b)
            b = boss[b]
        if b != -1 and state[b] == 1:
            return "following boss never loops"
        for q in path:
            state[q] = 2
    if not isinstance(pairs, list) or not 1 <= len(pairs) <= 5 * 10**4:
        return "1 ≤ pairs.length ≤ 5·10⁴"
    for p in pairs:
        if not (isinstance(p, list) and len(p) == 2 and all(type(x) is int and 0 <= x < n for x in p)):
            return "pairs[j] = [a, b] with 0 ≤ a, b < n"
    return None
