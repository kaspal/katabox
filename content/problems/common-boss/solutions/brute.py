def common_boss(boss: list[int], pairs: list[list[int]]) -> list[int]:
    # Obviously correct: list everyone above-or-equal a (closest first) and take the
    # first one who is also above-or-equal b.
    def chain(v):
        out = []
        while v >= 0:
            out.append(v)
            v = boss[v]
        return out

    return [next(x for x in chain(a) if x in chain(b)) for a, b in pairs]
