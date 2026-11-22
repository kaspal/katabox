def fewest_blocks(plays: list[int], requests: list[list[int]]) -> list[int]:
    # Obviously correct: the fewest blocks for every prefix of the request, trying every
    # last block.
    out = []
    for a, b in requests:
        part = plays[a : b + 1]
        best = [0] + [None] * len(part)
        for end in range(1, len(part) + 1):
            for begin in range(end):
                block = part[begin:end]
                if len(set(block)) == len(block):
                    cand = best[begin] + 1
                    if best[end] is None or cand < best[end]:
                        best[end] = cand
        out.append(best[len(part)])
    return out
