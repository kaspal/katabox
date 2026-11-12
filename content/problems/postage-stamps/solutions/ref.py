def exact_postage(values: list[int], target: int) -> list[int]:
    n = len(values)
    suffix = [0] * (n + 1)
    for i in range(n - 1, -1, -1):
        suffix[i] = suffix[i + 1] + values[i]
    best: list[int] | None = None
    chosen: list[int] = []

    def search(i: int, missing: int) -> None:
        nonlocal best
        if missing == 0:
            if best is None or len(chosen) < len(best):
                best = chosen[:]
            return
        if i == n or missing > suffix[i]:
            return
        if best is not None and len(chosen) + 1 >= len(best):
            return
        if values[i] <= missing:
            chosen.append(i)
            search(i + 1, missing - values[i])
            chosen.pop()
        search(i + 1, missing)

    search(0, target)
    return best if best is not None else []
