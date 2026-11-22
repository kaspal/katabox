def longest_pieces(ribbons: list[int], orders: list[int]) -> list[int]:
    # Obviously correct: for each order, try every length from the longest down.
    out = []
    for need in orders:
        best = 0
        for length in range(max(ribbons), 0, -1):
            if sum(r // length for r in ribbons) >= need:
                best = length
                break
        out.append(best)
    return out
