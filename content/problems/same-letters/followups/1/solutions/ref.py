def twins_after_changes(a: str, b: str, positions: list[int], letters: str) -> list[bool]:
    diff = [0] * 26  # count in a minus count in b, per letter
    for c in a:
        diff[ord(c) - 97] += 1
    for c in b:
        diff[ord(c) - 97] -= 1
    off = sum(1 for d in diff if d)  # letters whose counts differ
    tiles = list(b)
    out = []
    for p, c in zip(positions, letters):
        for letter, step in ((tiles[p], 1), (c, -1)):
            k = ord(letter) - 97
            before = diff[k] != 0
            diff[k] += step
            off += (diff[k] != 0) - before
        tiles[p] = c
        out.append(off == 0)
    return out
