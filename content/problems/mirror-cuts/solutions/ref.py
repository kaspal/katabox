def mirror_cuts(text: str) -> int:
    n = len(text)
    cuts = [i - 1 for i in range(n + 1)]
    for c in range(n):
        for l, r in ((c, c), (c, c + 1)):
            while l >= 0 and r < n and text[l] == text[r]:
                if cuts[l] + 1 < cuts[r + 1]:
                    cuts[r + 1] = cuts[l] + 1
                l -= 1
                r += 1
    return cuts[n]
