def prefix_counts(titles: list[str], typed: list[str]) -> list[int]:
    # Correct but O(N·Q·L): check every title for every search.
    out = []
    for p in typed:
        n = 0
        for t in titles:
            if t.startswith(p):
                n += 1
        out.append(n)
    return out
