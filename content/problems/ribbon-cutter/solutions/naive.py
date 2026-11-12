def longest_piece(ribbons: list[int], pieces: int) -> int:
    # Correct but O(n·M): try lengths from the longest roll downwards.
    length = max(ribbons)
    while length >= 1:
        count = 0
        for r in ribbons:
            count += r // length
        if count >= pieces:
            return length
        length -= 1
    return 0
