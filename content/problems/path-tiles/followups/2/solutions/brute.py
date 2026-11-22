def tilings_apart(n: int) -> int:
    # Obviously correct for small n: build every paving piece by piece.
    def count(left: int, last: int) -> int:
        if left == 0:
            return 1
        total = 0
        for piece in (1, 2, 3):
            if piece <= left and not (piece == 3 and last == 3):
                total += count(left - piece, piece)
        return total

    return count(n, 0) % 1_000_000_007
