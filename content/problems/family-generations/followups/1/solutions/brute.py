def kinship_distances(parent: list[int], pairs: list[list[int]]) -> list[int]:
    # Obviously correct: list each person's line up to the founder, find the first shared one.
    def line(v: int) -> list[int]:
        out = [v]
        while parent[v] != -1:
            v = parent[v]
            out.append(v)
        return out

    res = []
    for a, b in pairs:
        la, lb = line(a), line(b)
        common = next(x for x in la if x in lb)
        res.append(la.index(common) + lb.index(common))
    return res
