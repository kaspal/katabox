def is_int(s):
    return s.isascii() and s.isdigit()


def validate(text):
    lines = text.split("\n")
    if lines and lines[-1] == "":
        lines.pop()
    if not lines or not is_int(lines[0].strip()):
        return "the first line is T"
    t = int(lines[0])
    if not 1 <= t <= 1000:
        return "1 ≤ T ≤ 1000"
    pos = 1
    total = 0
    for case in range(1, t + 1):
        if pos >= len(lines) or not is_int(lines[pos].strip()):
            return f"park {case}: a line with N"
        n = int(lines[pos])
        pos += 1
        if not 1 <= n <= 20_000:
            return "1 ≤ N ≤ 20 000"
        total += n
        if total > 20_000:
            return "the N of all the parks add up to at most 20 000"
        if pos + n - 1 > len(lines):
            return f"park {case}: N − 1 trail lines"
        parent = list(range(n + 1))

        def find(v):
            while parent[v] != v:
                parent[v] = parent[parent[v]]
                v = parent[v]
            return v

        for line in lines[pos : pos + n - 1]:
            parts = line.split()
            if len(parts) != 2 or not all(is_int(p) for p in parts):
                return f"park {case}: each trail line holds two numbers a b"
            a, b = int(parts[0]), int(parts[1])
            if not (1 <= a <= n and 1 <= b <= n) or a == b:
                return "1 ≤ a, b ≤ N and a ≠ b"
            ra, rb = find(a), find(b)
            if ra == rb:
                return f"park {case}: the trails form a loop"
            parent[ra] = rb
        pos += n - 1
    if pos != len(lines):
        return "nothing may follow the last park"
    return None
