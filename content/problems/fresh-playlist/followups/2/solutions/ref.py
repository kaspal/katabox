def fewest_blocks(plays: list[int], requests: list[list[int]]) -> list[int]:
    n = len(plays)
    start = [0] * n
    last = {}
    left = 0
    for r, song in enumerate(plays):
        if last.get(song, -1) >= left:
            left = last[song] + 1
        last[song] = r
        start[r] = left
    nxt = [n] * (n + 1)  # first play after the longest fresh block from i
    r = 0
    for i in range(n):
        while r < n and start[r] <= i:
            r += 1
        nxt[i] = r
    jump = [nxt]
    while 1 << len(jump) <= n:
        prev = jump[-1]
        jump.append([prev[prev[i]] for i in range(n + 1)])
    out = []
    for a, b in requests:
        pos, blocks = a, 1
        for k in range(len(jump) - 1, -1, -1):
            if jump[k][pos] <= b:
                pos = jump[k][pos]
                blocks += 1 << k
        out.append(blocks)
    return out
