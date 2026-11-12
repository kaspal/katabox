def count_below(codes: list[int], queries: list[int]) -> list[int]:
    # Correct but O(n·q): walk the list from the start for every query.
    out = []
    for x in queries:
        i = 0
        while i < len(codes) and codes[i] < x:
            i += 1
        out.append(i)
    return out
