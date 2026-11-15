import re

INT = re.compile(r"[0-9]+")


def validate(text):
    lines = text.split("\n")
    if lines and lines[-1] == "":
        lines.pop()
    if len(lines) < 3 or not INT.fullmatch(lines[0]):
        return "n, the bosses, Q, then the questions"
    n = int(lines[0])
    if not 1 <= n <= 100_000:
        return "1 ≤ n ≤ 10⁵"
    bosses = lines[1].split()
    if len(bosses) != n - 1 or not all(INT.fullmatch(p) for p in bosses):
        return "the second line holds exactly n − 1 bosses (empty when n = 1)"
    boss = [0, 0] + [int(p) for p in bosses]
    for i in range(2, n + 1):
        if not 1 <= boss[i] <= n or boss[i] == i:
            return "1 ≤ boss ≤ n, and nobody is their own boss"
    # Everyone reaches the director: walk up with memo (no loops).
    state = [0] * (n + 1)  # 0 unknown, 1 on the current walk, 2 reaches the director
    state[1] = 2
    for start in range(2, n + 1):
        path = []
        x = start
        while state[x] == 0:
            state[x] = 1
            path.append(x)
            x = boss[x]
        if state[x] == 1:
            return "the chart has a loop: everyone must report to employee 1 in the end"
        for y in path:
            state[y] = 2
    if not INT.fullmatch(lines[2]):
        return "the line after the bosses is Q"
    q = int(lines[2])
    if not 1 <= q <= 100_000:
        return "1 ≤ Q ≤ 10⁵"
    if len(lines) != 3 + q:
        return "then exactly Q lines a b"
    for line in lines[3:]:
        parts = line.split(" ")
        if len(parts) != 2 or not all(INT.fullmatch(p) for p in parts):
            return "each question is a b"
        if not all(1 <= int(p) <= n for p in parts):
            return "1 ≤ a, b ≤ n"
    return None
