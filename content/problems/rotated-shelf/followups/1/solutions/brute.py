def free_numbers(shelf: list[int], queries: list[int]) -> list[int]:
    # Obviously correct: walk up from 1, skipping the shelf's numbers.
    taken = set(shelf)
    out = []
    for k in queries:
        x, seen = 0, 0
        while seen < k:
            x += 1
            if x not in taken:
                seen += 1
        out.append(x)
    return out
