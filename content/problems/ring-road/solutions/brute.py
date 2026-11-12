def ring_length(n: int, roads: list[list[int]]) -> int:
    # Obviously correct: a road is on the loop when its villages stay connected
    # without it; the loop has as many villages as roads.
    def connected(skip, a, b):
        seen, stack = {a}, [a]
        while stack:
            v = stack.pop()
            for k, (x, y) in enumerate(roads):
                if k == skip:
                    continue
                for p, q in ((x, y), (y, x)):
                    if p == v and q not in seen:
                        seen.add(q)
                        stack.append(q)
        return b in seen

    return sum(1 for k, (a, b) in enumerate(roads) if connected(k, a, b))
