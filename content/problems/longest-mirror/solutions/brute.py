def longest_mirror(text: str) -> str:
    # Obviously correct: every piece, longest first, earliest start first.
    n = len(text)
    for length in range(n, 0, -1):
        for i in range(n - length + 1):
            piece = text[i:i + length]
            if piece == piece[::-1]:
                return piece
    return ""
