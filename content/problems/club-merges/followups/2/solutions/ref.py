def clubs_with_undo(n: int, ops: list[list[int]]) -> list[int]:
    # Union by size without path compression, so a merge is undone by restoring two cells.
    parent = list(range(n))
    size = [1] * n
    clubs = n
    history: list[int] = []  # the absorbed root of each merge, or -1 for a merge that changed nothing

    def find(x: int) -> int:
        while parent[x] != x:
            x = parent[x]
        return x

    out = []
    for op in ops:
        if op[0] == 0:  # [0]: undo the latest merge still in effect
            if history:
                rb = history.pop()
                if rb != -1:
                    ra = parent[rb]
                    size[ra] -= size[rb]
                    parent[rb] = rb
                    clubs += 1
        else:  # [1, a, b]: merge
            ra, rb = find(op[1]), find(op[2])
            if ra == rb:
                history.append(-1)
            else:
                if size[ra] < size[rb]:
                    ra, rb = rb, ra
                parent[rb] = ra
                size[ra] += size[rb]
                clubs -= 1
                history.append(rb)
        out.append(clubs)
    return out
