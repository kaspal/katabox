def first_matches(words: list[str], prefixes: list[str]) -> list[str]:
    # Correct but O(n·q·L): scan every word for every prefix.
    out = []
    for p in prefixes:
        best = None
        for w in words:
            if w.startswith(p) and (best is None or w < best):
                best = w
        out.append(best if best is not None else "")
    return out
