def twins_after_changes(a: str, b: str, positions: list[int], letters: str) -> list[bool]:
    # Obviously correct: apply each change and compare sorted letters.
    tiles = list(b)
    out = []
    for p, c in zip(positions, letters):
        tiles[p] = c
        out.append(sorted(a) == sorted(tiles))
    return out
