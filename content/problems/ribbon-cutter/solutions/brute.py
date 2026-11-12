def longest_piece(ribbons: list[int], pieces: int) -> int:
    # Obviously correct: every length from 1 to the longest roll.
    best = 0
    for length in range(1, max(ribbons) + 1):
        if sum(r // length for r in ribbons) >= pieces:
            best = length
    return best
