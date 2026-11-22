from collections import deque


def fewest_insertions(text: str) -> int:
    # Obviously correct for short texts: search insertions breadth-first until balanced.
    s = "".join(c for c in text if c in "()")

    def balanced(t: str) -> bool:
        depth = 0
        for c in t:
            depth += 1 if c == "(" else -1
            if depth < 0:
                return False
        return depth == 0

    seen = {s}
    q = deque([(s, 0)])
    while q:
        t, d = q.popleft()
        if balanced(t):
            return d
        for i in range(len(t) + 1):
            for c in "()":
                u = t[:i] + c + t[i:]
                if u not in seen:
                    seen.add(u)
                    q.append((u, d + 1))
    return -1
