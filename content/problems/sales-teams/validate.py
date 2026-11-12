def validate(inp):
    boss, sales = inp["boss"], inp["sales"]
    if not isinstance(boss, list) or not all(type(b) is int for b in boss):
        return "boss is an array of integers"
    if not isinstance(sales, list) or not all(type(s) is int for s in sales):
        return "sales is an array of integers"
    n = len(boss)
    if not 1 <= n <= 10**5 or len(sales) != n:
        return "1 ≤ n ≤ 10⁵ and sales.length = boss.length"
    if not all(0 <= s <= 10**4 for s in sales):
        return "0 ≤ sales[i] ≤ 10⁴"
    if sum(1 for b in boss if b == -1) != 1:
        return "exactly one boss[i] is -1"
    if not all(b == -1 or 0 <= b < n for b in boss):
        return "every other boss[i] is between 0 and n − 1"
    state = [0] * n  # 0 unknown, 1 on the current path, 2 reaches the owner
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
    return None
