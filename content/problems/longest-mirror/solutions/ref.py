def longest_mirror(text: str) -> str:
    # Manacher's algorithm on "^#a#b#...#$": p[i] is the radius around position i, which
    # is also the length of the mirrored piece in `text`.
    t = "^#" + "#".join(text) + "#$"
    p = [0] * len(t)
    center = right = 0
    for i in range(1, len(t) - 1):
        if i < right:
            p[i] = min(right - i, p[2 * center - i])
        while t[i + p[i] + 1] == t[i - p[i] - 1]:
            p[i] += 1
        if i + p[i] > right:
            center, right = i, i + p[i]
    best, start = 0, 0
    for i in range(1, len(t) - 1):
        if p[i] > best:
            best, start = p[i], (i - p[i]) // 2
    return text[start:start + best]
