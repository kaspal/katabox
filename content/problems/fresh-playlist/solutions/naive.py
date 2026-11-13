def fresh_stretch(plays: list[int]) -> int:
    # Correct but O(n · answer): grow a stretch from every start until a repeat.
    best = 0
    for i in range(len(plays)):
        seen = set()
        for x in plays[i:]:
            if x in seen:
                break
            seen.add(x)
        best = max(best, len(seen))
    return best
