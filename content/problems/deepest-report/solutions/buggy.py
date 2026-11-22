def deepest_report(manager: list[int]) -> int:
    n = len(manager)
    depth = [0] * n

    def level(v: int) -> int:
        if manager[v] == -1:
            return 1
        if depth[v] == 0:
            depth[v] = level(manager[v]) + 1
        return depth[v]

    return max(level(v) for v in range(n))
